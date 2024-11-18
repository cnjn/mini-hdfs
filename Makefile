
ifdef OS
	CP = cmd /c copy
else
	CP = cp
endif


copy_files: cargo_build
	$(CP) log4rs.yml+target/release/client.exe dist


cargo_build:
	cargo build --release
