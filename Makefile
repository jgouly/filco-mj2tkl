BUILD_PATH=target/thumbv6m-none-eabi/release
ELF_PATH=${BUILD_PATH}/filco-mj2tkl
BIN_PATH=${ELF_PATH}.bin

build:
	xargo build --release
	arm-none-eabi-objcopy -O binary ${ELF_PATH} ${BIN_PATH}
	arm-none-eabi-size ${ELF_PATH}

clean:
	xargo clean
