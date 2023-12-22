## protobuf

## 两个入口

mian.rs 作为服务器，cli.rs 作为客服端

    [[bin]]
    name = "server"
    path = "src/main.rs"

    [[bin]]
    name = "cli"
    path = "src/cli.rs"

## protobuf .proto 文件

在根目录同级创建protofile文件夹，创建store.prote 文件，然后再根目录创建build.rs 文件，代码如下：监听protefile/store.proto 文件并生成对应的rs文件

    use std::env;
    use std::path::PathBuf;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let proto_file = "./protofile/store.proto";
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        tonic_build::configure()
            .protoc_arg("--experimental_allow_proto3_optional") // for older systems
            .build_client(true)
            .build_server(true)
            .file_descriptor_set_path(out_dir.join("store_descriptor.bin"))
            .out_dir("./src")
            .compile(&[proto_file], &["proto"])?;

        Ok(())
    }
