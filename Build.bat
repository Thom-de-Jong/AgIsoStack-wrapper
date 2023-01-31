@echo off

echo Building ISO11783-CAN-Stack...
cd ".\ISO11783-CAN-Stack"
git reset --hard HEAD
git pull
cmake -S . -B build -G "MinGW Makefiles" -DCAN_DRIVER=WindowsPCANBasic
cmake --build build
cd ..
echo Building ISO11783-CAN-Stack complete.

copy ".\ISO11783-CAN-Stack\build\isobus\libIsobus.a" ".\isobus-plus-plus-sys\libs\libIsobus.a"
robocopy .\ISO11783-CAN-Stack\isobus\include\isobus\isobus\ .\isobus-plus-plus-sys\headers\ /MIR /PURGE /IT

echo Building isobus-plus-plus-sys...
cd ".\isobus-plus-plus-sys"
cargo build
cd ..
echo Building isobus-plus-plus-sys complete.

echo Building isobus-plus-plus...
cd ".\isobus-plus-plus"
cargo build
cargo build --examples
cd ..
echo Building isobus-plus-plus complete.

pause