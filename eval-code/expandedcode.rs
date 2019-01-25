#![feature(prelude_import)]                                                                                                                   
#![no_std]                                                                                                                                    
#[prelude_import]                                                                                                                             
use ::std::prelude::v1::*;                                                                                                                    
#[macro_use]                                                                                                                                  
extern crate std;                                                                                                                             
// #![no_std]                                                                                                                                 
// #![feature(lang_items, start)]                                                                                                             
extern crate core;                                                                                                                            
extern crate pest;                                                                                                                            
#[macro_use]                                                                                                                                  
extern crate pest_derive;                                                                                                                     
                                                                                                                                              
use pest::Parser;                                                                                                                             
use std::time::Instant;                                                                                                                       
                                                                                                                                              
#[allow(dead_code)]                                                                                                                           
struct Command {                                                                                                                              
    driver_number: usize,                                                                                                                     
    subdriver_number: usize,                                                                                                                  
    arg0: usize,                                                                                                                              
    arg1: usize,                                                                                                                              
}                                                                                                                                             
#[allow(dead_code, non_camel_case_types)]                                                                                                     
#[structural_match]                                                                                                                           
#[rustc_copy_clone_marker]                                                                                                                    
pub enum CommandRuleEnum { CommandRule, }                                                                                                     
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::clone::Clone for CommandRuleEnum {                                                                                                
    #[inline]                                                                                                                                 
    fn clone(&self) -> CommandRuleEnum { { *self } }                                                                                          
}                                                                                                                                             
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::marker::Copy for CommandRuleEnum { }                                                                                              
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::fmt::Debug for CommandRuleEnum {                                                                                                  
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {                                                                      
        match (&*self,) {                                                                                                                     
            (&CommandRuleEnum::CommandRule,) => {                                                                                             
                let mut debug_trait_builder = f.debug_tuple("CommandRule");                                                                   
                debug_trait_builder.finish()                                                                                                  
            }                                                                                                                                 
        }                                                                                                                                     
    }                                                                                                                                         
}                                                                                                                                             
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::cmp::Eq for CommandRuleEnum {                                                                                                     
    #[inline]                                                                                                                                 
    #[doc(hidden)]                                                                                                                            
    fn assert_receiver_is_total_eq(&self) -> () { { } }                                                                                       
}                                                                                                                                             
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::hash::Hash for CommandRuleEnum {                                                                                                  
    fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {                                                                         
        match (&*self,) { _ => { } }                                                                                                          
    }                                                                                                                                         
}                                                                                                                                             
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::cmp::Ord for CommandRuleEnum {                                                                                                    
    #[inline]                                                                                                                                 
    fn cmp(&self, other: &CommandRuleEnum) -> ::std::cmp::Ordering {                                                                          
        match (&*self, &*other) { _ => ::std::cmp::Ordering::Equal, }                                                                         
    }                                                                                                                                         
}                                                                                                                                             
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::cmp::PartialEq for CommandRuleEnum {                                                                                              
    #[inline]                                                                                                                                 
    fn eq(&self, other: &CommandRuleEnum) -> bool {                                                                                           
        match (&*self, &*other) { _ => true, }                                                                                                
    }                                                                                                                                         
}                                                                                                                                             
#[automatically_derived]                                                                                                                      
#[allow(unused_qualifications)]                                                                                                               
#[allow(dead_code, non_camel_case_types)]                                                                                                     
impl ::std::cmp::PartialOrd for CommandRuleEnum {                                                                                             
    #[inline]                                                                                                                                 
    fn partial_cmp(&self, other: &CommandRuleEnum)                                                                                            
     -> ::std::option::Option<::std::cmp::Ordering> {                                                                                         
        match (&*self, &*other) {                                                                                                             
            _ => ::std::option::Option::Some(::std::cmp::Ordering::Equal),                                                                    
        }                                                                                                                                     
    }                                                                                                                                         
}                                                                                                                                             
impl ::pest::Parser<CommandRuleEnum> for Command {                                                                                            
    fn parse<'i>(input: &'i [u8])                                                                                                             
     -> ::core::result::Result<usize, ::pest::error::Error> {                                                                                 
        mod rules {                                                                                                                           
            use super::CommandRuleEnum;                                                                                                       
            #[inline]                                                                                                                         
            #[allow(non_snake_case, unused_variables)]                                                                                        
            pub fn CommandRule(state: ::pest::ParserState<CommandRuleEnum>)                                                                   
             -> ::pest::ParseResult<::pest::ParserState<CommandRuleEnum>> {                                                                   
                state.rule(CommandRuleEnum::CommandRule,                                                                                      
                           |state|                                                                                                            
                               {                                                                                                              
                                   state.sequence(|state|                                                                                     
                                                      {                                                                                       
                                                          self::USIZE(state).and_then(|state|                                                 
                                                                                          {                                                   
                                                                                              self::skip(state)                               
                                                                                          }).and_then(|state|                                 
                                                                                                          {                                   
                                                                                                              self::USIZE(state)              
                                                                                                          }).and_then(|state|                 
                                                                                                                          {                   
                                                                                                                              self::skip(state)
                                                                                                                          }).and_then(|state| 
                                                                                                                                          {   
                                                                                                                                              self::USIZE(state)
                                                                                                                                          }).and_then(|state|
                                                                                                                                                          {
                                                                                                                                                              self::skip(state)
                                                                                                                                                          }).and_then(|state|
                                                                                                                                                                          {
                                                                                                                                                                              self::USIZE(state)
                                                                                                                                                                          })
                                                      })                                                                                      
                               })                                                                                                             
            }                                                                                                                                 
            #[inline]                                                                                                                         
            #[allow(dead_code, non_snake_case, unused_variables)]                                                                             
            fn USIZE(state: ::pest::ParserState<CommandRuleEnum>)                                                                             
             -> ::pest::ParseResult<::pest::ParserState<CommandRuleEnum>> {                                                                   
                state.match_usize()                                                                                                           
            }                                                                                                                                 
            #[inline]                                                                                                                         
            #[allow(dead_code, non_snake_case, unused_variables)]                                                                             
            fn skip(state: ::pest::ParserState<CommandRuleEnum>)                                                                              
             -> ::pest::ParseResult<::pest::ParserState<CommandRuleEnum>> {                                                                   
                Ok(state)                                                                                                                     
            }                                                                                                                                 
        }                                                                                                                                     
        ::pest::state(input, |state| { rules::CommandRule(state) })                                                                           
    }                                                                                                                                         
}                                                                                                                                             
impl Command {                                                                                                                                
    fn create(le: bool, offset: &mut usize, bytes: &[u8]) -> Command {                                                                        
        Command{driver_number: ::pest::reader::usize(le, offset, bytes),                                                                      
                subdriver_number: ::pest::reader::usize(le, offset, bytes),                                                                   
                arg0: ::pest::reader::usize(le, offset, bytes),                                                                               
                arg1: ::pest::reader::usize(le, offset, bytes),}                                                                              
    }                                                                                                                                         
    fn size() -> usize { 16usize }                                                                                                            
    fn parse_and_create<'i>(le: bool, input: &'i [u8]) -> Command {                                                                           
        let mut offset: usize = 0;                                                                                                            
        Command::parse(input).unwrap_or_else(|e|                                                                                              
                                                 // #[derive(Parser)]                                                                         
                                                 // #[allow(dead_code)]                                                                       
                                                 // struct Child {                                                                            
                                                 //   first_field: u16,                                                                       
                                                 //   second_field: u16,                                                                      
                                                 // }                                                                                         
                                                                                                                                              
                                                 // #[derive(Parser)]                                                                         
                                                 // #[allow(dead_code)]                                                                       
                                                 // struct TypesParser {                                                                      
                                                 //   first_field: Child,                                                                     
                                                 //   second_field: u16,                                                                      
                                                 // }                                                                                         
                                                                                                                                              
                                                 //use core::panic::PanicInfo;                                                                
                                                                                                                                              
                                                 // #[panic_handler]                                                                          
                                                 // fn panic(_info: &PanicInfo) -> ! {                                                        
                                                 //     loop {}                                                                               
                                                 // }                                                                                         
                                                                                                                                              
                                                 // #[lang = "eh_personality"]                                                                
                                                 // pub extern "C" fn eh_personality() {}                                                     
                                                                                                                                              
                                                 // #[start]                                                                                  
                                                 // fn start(_argc: isize, _argv: *const *const u8) -> isize {                                
                                                 //   let input = [9, 0, 1, 0, 8, 0, 0, 0];                                                   
                                                 //   let t = TypesParser::parse_and_create(true, &input);                                    
                                                 //   0                                                                                       
                                                 // }                                                                                         
                                                                                                                                              
                                                                                                                                              
                                                 // let mut offset: usize = 0;                                                                
                                                                                                                                              
                                                 // let c = Child { first_field: 0, second_field: 0, third_field: 0, fourth_field: 0 };       
                                                 // Child::validate(&c as *const Child as *const u8);                                         
                                                                                                                                              
                                                 // println!("{:?}", t.first_field.first_field);                                              
                                                 // println!("{:?}", t.first_field.second_field);                                             
                                                 // println!("{:?}", t.first_field.third_field);                                              
                                                 // println!("{:?}", t.second_field);                                                         
                                                                                                                                              
                                                 // Child::validate(&t as *const Child as *const u8);                                         
                                                                                                                                              
                                                                                                                                              
                                                 // println!("{} {} {} {}", end.as_secs(), end.subsec_millis(), end.subsec_micros(), end.subsec_nanos());
                                                 {                                                                                            
                                                     ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1_formatted(&[""],                    
                                                                                                                    &match (&e,)              
                                                                                                                         {                    
                                                                                                                         (arg0,)              
                                                                                                                         =>                   
                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                     },                       
                                                                                                                    &[::std::fmt::rt::v1::Argument{position:
                                                                                                                                                       ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                   format:
                                                                                                                                                       ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          ::std::fmt::rt::v1::Count::Implied,},}]),
                                                                           &("src/main.rs",                                                   
                                                                             12u32,                                                           
                                                                             10u32))                                                          
                                                 });                                                                                          
        Command::create(le, &mut offset, input)                                                                                               
    }                                                                                                                                         
    fn sub_parse_and_create<'i>(le: bool, offset: &mut usize, input: &'i [u8])                                                                
     -> Command {                                                                                                                             
        Command::parse(input).unwrap_or_else(|e|                                                                                              
                                                 {                                                                                            
                                                     ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1_formatted(&[""],                    
                                                                                                                    &match (&e,)              
                                                                                                                         {                    
                                                                                                                         (arg0,)              
                                                                                                                         =>                   
                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                     },                       
                                                                                                                    &[::std::fmt::rt::v1::Argument{position:
                                                                                                                                                       ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                   format:
                                                                                                                                                       ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          ::std::fmt::rt::v1::Count::Implied,},}]),
                                                                           &("src/main.rs",                                                   
                                                                             12u32,                                                           
                                                                             10u32))                                                          
                                                 });                                                                                          
        Command::create(le, offset, input)                                                                                                    
    }                                                                                                                                         
    fn validate(ptr: *const u8) {                                                                                                             
        unsafe {                                                                                                                              
            let slice = core::slice::from_raw_parts(ptr, Command::size());                                                                    
            Command::parse_and_create(true, &slice);                                                                                          
        }                                                                                                                                     
    }                                                                                                                                         
}                                                                                                                                             
fn main() {                                                                                                                                   
    for _ in 0..200 {                                                                                                                         
        let start = Instant::now();                                                                                                           
        let input = [9, 0, 1, 0, 8, 0, 0, 0, 2, 2, 2, 2, 0, 9, 0, 9];                                                                         
        let t = Command::parse(&input);                                                                                                       
        let end = start.elapsed();                                                                                                            
        {                                                                                                                                     
            ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["", "\n"],                                                                 
                                                                 &match (&end.subsec_nanos(),)                                                
                                                                      {                                                                       
                                                                      (arg0,)                                                                 
                                                                      =>                                                                      
                                                                      [::std::fmt::ArgumentV1::new(arg0,                                      
                                                                                                   ::std::fmt::Display::fmt)],                
                                                                  },                                                                          
                                                                 &[::std::fmt::rt::v1::Argument{position:                                     
                                                                                                    ::std::fmt::rt::v1::Position::At(0usize), 
                                                                                                format:                                       
                                                                                                    ::std::fmt::rt::v1::FormatSpec{fill:      
                                                                                                                                       ' ',   
                                                                                                                                   align:     
                                                                                                                                       ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                   flags:     
                                                                                                                                       0u32,  
                                                                                                                                   precision: 
                                                                                                                                       ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                   width:     
                                                                                                                                       ::std::fmt::rt::v1::Count::Implied,},}]));
        };                                                                                                                                    
    }                                                                                                                                         
}                                                                                                                                             
