// I DONT KNOW IF THIS WORKS
// I haven't ran this file yet.
// I tested the C in tests/capi.rs via cargo ctest.

#include "example_project/example_project.h"
#include <stdio.h>

int main() {
    const char* program_name_cstr = "/bin/cat";
    OsString_extern program_name_osstring;
    OsString_new_extern(
        program_name_cstr,
        strlen(program_name_cstr),
        &program_name_osstring);
    
    const char* arg1_cstr = "/Users/verdagon/hello.txt";
    OsString_extern arg1_osstring;
    OsString_new_extern(
        arg1_cstr,
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

    // ExampleProjectOddCounter *counter = example_project_oddcounter_new(4);
    // if (counter) {
    //     printf("Unexpected success\n");
    //     return 1;
    // }
    // counter = example_project_oddcounter_new(5);
    // if (!counter) {
    //     printf("Error creating ExampleProjectOddCounter\n");
    //     return 1;
    // }
    // example_project_oddcounter_increment(counter);
    // uint32_t result = example_project_oddcounter_get_current(counter);
    // example_project_oddcounter_free(counter);
    // if (result == 7) {
    //     return 0;
    // } else {
    //     printf("Error: unexpected result: %d\n", result);
    //     return 1;
    // }
}
