//
// Created by fulva on 7/18/2026.
//

#include <iostream>
#include <ostream>
#include <rust/cxx.h>
#include <rscxx/src/lib.rs.h>


int main(int argc, char *argv[]) {
    auto concatted = string_concat("Hello ", "world!");
    std::cout << concatted << std::endl;
}
