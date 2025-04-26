
// #[macro_export]
// macro_rules! get_cmd_out {
//     ($($args:expr),+) => {{
//        let r= duct::cmd!($($args),+).stdout_capture().run().expect("failed to run cmd");
//        let stdout=r.stdout;
//        let s=stdout.to_owned();
//        let rr=move || str::from_utf8(s.as_ref());
       
//        rr
//    }};
// }
pub fn get_cmd_out(program:String, args:Vec<String>) ->anyhow::Result<String>{
       let r= duct::cmd(program,args).stdout_capture().run()?;
       let stdout=r.stdout;
       let s=stdout.to_owned();
       let rr=str::from_utf8(s.as_ref());
       
      Ok( rr?.to_string().trim().to_string())
   }
