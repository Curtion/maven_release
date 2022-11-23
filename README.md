# 说明

该工具用于多服务之间的依赖更新。指定一个服务名，该工具在当前目录下搜索相应文件夹，修改其中`pom.xml`的`verison`的字段，同时修改父级`pom.xml`中的`version`字段。

然后修改该服务下的所有子服务的`pom.xml`中的`parent`->`version`字段为父级`version`字段。

最后查询所有服务，找到引用了指定服务名的服务，修改`pom.xml`中的`dependencyManagement`的`version`字段，同时修改子服务中的`pom.xml`中的`parent`->`version`字段。

# 使用说明

1. 下载 `bin`目录下的exe文件
2. 配置 `环境变量` (可选)
3. 执行 `maven_release.exe`即可。

如果没有配置环境变量，需要把exe放置到文件夹内执行，或者通过`--path`指定工作目录

如果配置了环境变量，可以直接在任意目录执行`maven_release.exe`，会自动获取当前目录