use anyhow::{Context, Result};
use colored::*;
use serde_json::Value;
use std::collections::HashMap;

pub fn pretty_print_json(json_str: &str) -> Result<String> {
    let value: Value = serde_json::from_str(json_str)
        .with_context(|| "Invalid JSON format")?;
    
    let pretty = serde_json::to_string_pretty(&value)
        .with_context(|| "Failed to format JSON")?;
    
    Ok(colorize_json(&pretty))
}

fn colorize_json(json: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut chars = json.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '"' if !escaped => {
                in_string = !in_string;
                if in_string {
                    result.push_str(&ch.to_string().green().to_string());
                } else {
                    result.push_str(&ch.to_string().green().to_string());
                }
            }
            '\\' if in_string => {
                escaped = !escaped;
                result.push_str(&ch.to_string().green().to_string());
            }
            _ if in_string => {
                escaped = false;
                result.push_str(&ch.to_string().green().to_string());
            }
            ':' => {
                result.push_str(&ch.to_string().yellow().to_string());
            }
            '{' | '}' | '[' | ']' => {
                result.push_str(&ch.to_string().cyan().bold().to_string());
            }
            ',' => {
                result.push_str(&ch.to_string().white().to_string());
            }
            _ if ch.is_ascii_digit() || ch == '.' || ch == '-' => {
                let mut number = String::new();
                number.push(ch);
                
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' || next_ch == 'e' || next_ch == 'E' || next_ch == '+' || next_ch == '-' {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                result.push_str(&number.blue().to_string());
            }
            _ if ch.is_alphabetic() => {
                let mut word = String::new();
                word.push(ch);
                
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphabetic() {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                match word.as_str() {
                    "true" | "false" => result.push_str(&word.magenta().to_string()),
                    "null" => result.push_str(&word.red().to_string()),
                    _ => result.push_str(&word),
                }
            }
            _ => {
                escaped = false;
                result.push(ch);
            }
        }
    }
    
    result
}

pub fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.2}s", ms as f64 / 1000.0)
    } else {
        format!("{:.2}m", ms as f64 / 60000.0)
    }
}

pub fn format_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

pub fn is_valid_json(json_str: &str) -> bool {
    serde_json::from_str::<Value>(json_str).is_ok()
}

pub fn json_path_extract(json_str: &str, path: &str) -> Result<Value> {
    let value: Value = serde_json::from_str(json_str)?;
    let parts: Vec<&str> = path.split('.').collect();
    
    let mut current = &value;
    for part in parts {
        if part.is_empty() {
            continue;
        }
        
        // Handle array indices
        if let Ok(index) = part.parse::<usize>() {
            current = current.get(index)
                .with_context(|| format!("Array index {} not found", index))?;
        } else {
            current = current.get(part)
                .with_context(|| format!("Key '{}' not found", part))?;
        }
    }
    
    Ok(current.clone())
}

pub fn parse_headers_string(headers_str: &str) -> Result<HashMap<String, String>> {
    let mut headers = HashMap::new();
    
    for line in headers_str.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(
                key.trim().to_string(),
                value.trim().to_string()
            );
        } else {
            anyhow::bail!("Invalid header format: {}", line);
        }
    }
    
    Ok(headers)
}

pub fn status_message(status_code: u16) -> String {
    match status_code {
        200 => "OK".to_string(),
        201 => "Created".to_string(),
        204 => "No Content".to_string(),
        400 => "Bad Request".to_string(),
        401 => "Unauthorized".to_string(),
        403 => "Forbidden".to_string(),
        404 => "Not Found".to_string(),
        500 => "Internal Server Error".to_string(),
        502 => "Bad Gateway".to_string(),
        503 => "Service Unavailable".to_string(),
        _ => "Unknown Status".to_string(),
    }
}

pub fn status_indicator(status_code: u16) -> String {
    let status_str = status_code.to_string();
    match status_code {
        200..=299 => status_str.green().bold().to_string(),
        300..=399 => status_str.yellow().bold().to_string(),
        400..=499 => status_str.red().bold().to_string(),
        500..=599 => status_str.red().bold().on_white().to_string(),
        _ => status_str.white().to_string(),
    }
}