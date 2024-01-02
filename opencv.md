## 安装opencv

### 前置条件

需要安装cmake,visual studio 2019,llvm ,opencv
cmake ,visual studio 2019
直接官网下载即可

####

llvm: github下载llvm win64安装exe文件，
llvm-config:llvm 官网下载源码，解压后cd到llvm文件夹,<code>cmake . </code>编译生成sln文件，使用visual studio 打开sln文件，编译生成llvm-config.exe，将llvm-config.exe 拷贝到llvm安装目录的bin文件夹下
配置环境变量 LLVM_CONFIG_PATH,LIBCLANG_PATH

> <https://blog.csdn.net/u013195275/article/details/106871312>
