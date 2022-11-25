//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::Feeder;
use crate::element_list::{ControlOperator, RedirectOp};

pub fn scanner_until_escape(text: &Feeder, from: usize, to: &str) -> usize {
    let mut pos = from;
    let mut escaped = false;
    for ch in text.chars_after(from) {
        if escaped || ch == '\\' {
            escaped = !escaped;
        }else if let Some(_) = to.find(ch) {
            break;
        };
        pos += ch.len_utf8();
    }
    pos
}

pub fn scanner_blank(text: &Feeder, from: usize) -> usize {
    let mut pos = from;
    for ch in text.chars_after(from) {
        if let Some(_) = " \t".find(ch) {
            pos += ch.len_utf8();
        }else{
            break;
        };
    }
    pos
}

pub fn scanner_until(text: &Feeder, from: usize, to: &str) -> usize {
    let mut pos = from;
    for ch in text.chars_after(from) {
        if let Some(_) = to.find(ch) {
            break;
        };
        pos += ch.len_utf8();
    }
    pos
}

impl Feeder {
    pub fn scanner_name_or_parameter(&mut self, from: usize) -> usize {
        let ans = self.scanner_parameter(from);
    
        if ans == 0 {
            self.scanner_name(from)
        }else{
            ans
        }
    }
    
    pub fn scanner_number(&mut self, from: usize) -> usize {
        let mut pos = from;
        for ch in self.chars_after(from) {
            if ch < '0' || '9' < ch {
                break;
            }
            pos += 1;
        }
        pos
    }

    pub fn scanner_parameter(&mut self, from: usize) -> usize {
        if self.len() < from {
            return from;
        }
    
        if "?*@$#!-:".chars().any(|c| c == self.nth(from)) { //special parameters
            return from+1;
        };
    
        self.scanner_number(from)
    }


    pub fn scanner_redirect(&mut self) -> (usize, Option<RedirectOp> ) {
        if self.starts_with("<<<") {
            return (3, Some(RedirectOp::HereStr));
        }else if self.starts_with("&>>") {
            return (3, Some(RedirectOp::AndAppend));
        }else if self.starts_with(">>") {
            return (2, Some(RedirectOp::Append));
        }else if self.starts_with("<<") {
            return (2, Some(RedirectOp::HereDoc));
        }else if self.starts_with(">&") {
            return (2, Some(RedirectOp::OutputAnd));
        }else if self.starts_with("&>") {
            return (2, Some(RedirectOp::AndOutput));
        }else if self.starts_with("<>") {
            return (2, Some(RedirectOp::InOut));
        }else if self.starts_with(">") {
            return (1, Some(RedirectOp::Output));
        }else if self.starts_with("<") {
            return (1, Some(RedirectOp::Input));
        }
        (0, None)
    }

    pub fn scanner_name(&mut self, from: usize) -> usize {
        if self.len() <= from {
            return from;
        }
    
        let h = &self.nth(0);
        if !((*h >= 'A' && *h <= 'Z') || (*h >= 'a' && *h <= 'z') || *h == '_') {
            return from;
        }
    
        if self.len() == from+1 {
            return from+1;
        }
    
        let mut ans = from+1;
        for c in self.chars_after(from+1) {
            if !((c >= '0' && c <= '9') || (c >= 'A' && c <= 'Z') 
            || (c >= 'a' && c <= 'z') || c == '_') {
                break;
            }
            ans += 1;
        }
    
        return ans;
    }

    pub fn scanner_control_op(&mut self) -> (usize, Option<ControlOperator> ) {
        let mut op = None;
        let mut pos = 0;
    
        if self.len() > 2  {
            pos = 3;
            op = if self.starts_with(";;&") {
                Some(ControlOperator::SemiSemiAnd)
            }else{
                None
            };
        }
    
        if op == None && self.len() > 1  {
            pos = 2;
            op = if self.starts_with("||") {
                Some(ControlOperator::Or)
            }else if self.starts_with("&&") {
                Some(ControlOperator::And)
            }else if self.starts_with(";;") {
                Some(ControlOperator::DoubleSemicolon)
            }else if self.starts_with(";&") {
                Some(ControlOperator::SemiAnd)
            }else if self.starts_with("|&") {
                Some(ControlOperator::PipeAnd)
            }else{
                None
            };
    
        }
    
        if op == None && self.len() > 0  {
            pos = 1;
            if self.starts_with("&") {
                if self.len() > 1 && self.nth(1) == '>' {
                    return (0, None)
                }
                return (1, Some(ControlOperator::BgAnd));
            } else if self.starts_with("\n") {
                return (1, Some(ControlOperator::NewLine));
            } else if self.starts_with("|") {
                return (1, Some(ControlOperator::Pipe));
            } else if self.starts_with(";") {
                return (1, Some(ControlOperator::Semicolon));
            } else if self.starts_with("(") {
                return (1, Some(ControlOperator::LeftParen));
            } else if self.starts_with(")") {
                return (1, Some(ControlOperator::RightParen));
            }
        }
    
        if op != None && self.len() > pos && self.nth(pos) == '\n' {
            pos += 1;
        }
    
        if op != None{
            return (pos, op);
        }
    
    
        (0 , None)
    }

    pub fn scanner_comment(&mut self) -> usize {
        //if text.len() > from && text.nth_is(from, "#") {
        if self.starts_with("#") {
            return scanner_until(self, 0, "\n");
        }
    
        0
    }

    pub fn scanner_integer(&mut self) -> usize {
        if self.len() == 0 {
            return 0;
        }
    
        let mut pos = 0;
        let mut minus = false;
        if self.starts_with("-") {
            pos += 1;
            minus = true;
        }
    
        for ch in self.chars_after(pos) {
            if ch < '0' || ch > '9' {
                break;
            }
    
            pos += 1;
        }
    
        if minus && pos == 1 {
            0
        }else{
            pos
        }
    }
}
