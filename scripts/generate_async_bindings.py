#!/usr/bin/env python3
"""
自动生成异步客户端绑定的脚本

从同步客户端的方法签名自动生成对应的异步版本
"""

import re
import sys
from pathlib import Path

def extract_methods(file_path):
    """提取文件中的所有方法签名"""
    content = Path(file_path).read_text()

    # 匹配方法定义：/// 注释 + #[pyo3] + fn 方法名
    pattern = r'((?:///[^\n]*\n\s*)*)((?:#\[pyo3[^\]]*\]\n\s*)*)(fn\s+(\w+)\s*\([^)]*\)[^{]*)'

    methods = []
    for match in re.finditer(pattern, content, re.MULTILINE):
        doc_comment = match.group(1).strip()
        pyo3_attrs = match.group(2).strip()
        signature = match.group(3).strip()
        method_name = match.group(4)

        methods.append({
            'name': method_name,
            'doc': doc_comment,
            'attrs': pyo3_attrs,
            'signature': signature
        })

    return methods

def convert_to_async(method):
    """将同步方法签名转换为异步版本"""
    sig = method['signature']
    name = method['name']

    # 跳过特殊方法
    if name in ['new', 'rest_client', 'block_on_allow_threads']:
        return None

    # 转换签名：添加 <'py> 和 py: Python<'py> 参数
    # fn method(&self, param: Type) -> PyResult<T>
    # 转换为
    # fn method<'py>(&self, py: Python<'py>, param: Type) -> PyResult<Bound<'py, PyAny>>

    # 替换 &self 后添加 py 参数
    sig = re.sub(r'&self,?\s*', "&self, py: Python<'py>, ", sig)
    if '&self' in sig and 'py: Python' not in sig:
        sig = sig.replace('&self', "&self, py: Python<'py>")

    # 添加生命周期参数
    sig = re.sub(r'fn\s+(\w+)\s*\(', r"fn \1<'py>(", sig)

    # 转换返回类型为 Bound<'py, PyAny>（处理嵌套的泛型）
    while 'PyResult<' in sig and 'Bound<' not in sig:
        sig = re.sub(r'PyResult<[^<>]*>', "PyResult<Bound<'py, PyAny>>", sig)

    # 转换参数类型：&str -> String, Option<&str> -> Option<String>
    sig = re.sub(r':\s*&str', ': String', sig)
    sig = re.sub(r'Option<&str>', 'Option<String>', sig)

    return sig

def generate_async_method(method, module_name):
    """生成异步方法的完整代码"""
    async_sig = convert_to_async(method)
    if not async_sig:
        return None

    name = method['name']
    doc = method['doc'].replace('。', '（异步）。') if method['doc'] else f'/// {name}（异步）。'
    attrs = method['attrs']

    # 生成方法体：调用对应的实现模块
    impl_call = f"{module_name}_impl::async_api::{name}(self, py"

    # 提取参数名（更全面的类型匹配）
    params = re.findall(r'(\w+):\s*(?:String|Option<String>|u32|Option<u32>|bool|Option<bool>|i64|Option<i64>|f64|Option<f64>)', async_sig)
    for param in params:
        if param not in ['self', 'py']:
            impl_call += f", {param}"
    impl_call += ")"

    code = f"""    {doc}
    {attrs}
    {async_sig} {{
        {impl_call}
    }}
"""
    return code

def main():
    if len(sys.argv) < 3:
        print("用法: python generate_async_bindings.py <sync_file> <module_name>")
        print("示例: python generate_async_bindings.py crates/okx-py/src/client/account.rs account")
        sys.exit(1)

    sync_file = sys.argv[1]
    module_name = sys.argv[2]

    print(f"从 {sync_file} 提取方法...")
    methods = extract_methods(sync_file)
    print(f"找到 {len(methods)} 个方法")

    print("\n生成异步版本：\n")
    print("=" * 80)

    for method in methods:
        async_code = generate_async_method(method, module_name)
        if async_code:
            print(async_code)

    print("=" * 80)
    print(f"\n✅ 生成完成！共 {len([m for m in methods if convert_to_async(m)])} 个异步方法")

if __name__ == '__main__':
    main()
