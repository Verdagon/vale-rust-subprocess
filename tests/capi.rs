#[cfg(feature = "capi")]
mod capi {

    use inline_c::assert_c;

    #[test]
    fn test_capi() {
        (assert_c! {
        #include <example_project.h>
        #include <stdio.h>
        #include <string.h>

        int main() {
            const char* program_name_cstr = "/bin/cat";
            OsString_extern program_name_osstring;
            OsString_new_extern(
                (const int8_t*)program_name_cstr,
                strlen(program_name_cstr),
                &program_name_osstring);
            
            const char* arg1_cstr = "/Users/verdagon/hello.txt";
            OsString_extern arg1_osstring;
            OsString_new_extern(
                (const int8_t*)arg1_cstr,
                strlen(arg1_cstr),
                &arg1_osstring);
            
            Vec_Ref_OsString_extern argv;
            Vec_Ref_OsString_new(&argv);

            Vec_Ref_OsString_push(&argv, &program_name_osstring);
            Vec_Ref_OsString_push(&argv, &arg1_osstring);

            PopenConfig_extern popen_config;
            PopenConfig_default_extern(&popen_config);

            Result_Popen_PopenError_extern result;
            Popen_create_extern(&argv, &popen_config, &result);

            bool is_ok = Result_Popen_PopenError_is_ok_extern(&result);

            if (!is_ok) {
              return 1;
            }
            return 0;
        }
            })
        .success();
    }
}
