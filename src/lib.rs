// 导入 wasm_bindgen 预导入
use wasm_bindgen::prelude::*;


// 定义一个函数，供JavaScript调用，计算数学表达式的值
#[wasm_bindgen]
pub fn calculate(expression: &str) -> Result<f64, String> {
    // 把输入的字符串按空格分割， 比如 "2 + 2" 分割成 ["2", "+", "2"]
    let parts: Vec<&str> = expression.split_whitespace().collect();

    // 检测输入是否正确，必须有3个部分 数字 运算符 数字
    if parts.len() != 3 {
        return Err("Invalid input".to_string());
    }

    // 解析第一个数字
    let left = parts[0].parse::<f64>().map_err(|_| "第一个数字无效".to_string())?;
    // 解析第二个数字
    let right = parts[2].parse::<f64>().map_err(|_| "第二个数字无效".to_string())?;

    // 解析运算符
    let operator = parts[1];
    
    // 根据运算符进行计算
    match operator {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0.0 { // 除数不能为0
                Err("除数不能为0".to_string())
            } else {
                Ok(left / right)
            }
        },
        _ => Err("Invalid operator".to_string()),
    }
}

