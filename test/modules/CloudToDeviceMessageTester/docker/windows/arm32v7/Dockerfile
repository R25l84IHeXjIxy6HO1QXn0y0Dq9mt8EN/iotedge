ARG base_tag=1.0.5-windows-arm32v7
ARG base_registry
FROM ${base_registry}/azureiotedge/azureiotedge-module-base:${base_tag}

ENV MODULE_NAME "CloudToDeviceMessageTester.dll"

ARG EXE_DIR=.

WORKDIR /app

COPY $EXE_DIR/ ./

CMD ["dotnet", "CloudToDeviceMessageTester.dll"]