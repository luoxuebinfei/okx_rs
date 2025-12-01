#!/bin/bash
# okx-py 测试运行脚本

set -e

# 颜色输出
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== OKX-PY 测试套件 ===${NC}"

# 检查虚拟环境
if [ ! -d "../../.venv" ]; then
    echo -e "${RED}错误: 未找到虚拟环境，请先运行: uv venv --python 3.12${NC}"
    exit 1
fi

# 激活虚拟环境
source ../../.venv/bin/activate

# 检查是否需要重新编译
echo -e "${YELLOW}检查是否需要重新编译...${NC}"
if [ ! -f "../../.venv/lib/python3.12/site-packages/okx_py.so" ] || [ "src/lib.rs" -nt "../../.venv/lib/python3.12/site-packages/okx_py.so" ]; then
    echo -e "${YELLOW}重新编译 okx-py...${NC}"
    cd ../..
    maturin develop --manifest-path crates/okx-py/Cargo.toml
    cd crates/okx-py
fi

# 运行测试
echo -e "${GREEN}运行测试...${NC}"
pytest tests/ -v "$@"

echo -e "${GREEN}测试完成！${NC}"
