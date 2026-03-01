#!/bin/bash

# 1. 配置信息 (根据实际情况修改)
DB_USER="postgres"
DB_HOST="localhost"
DB_PORT="5432"
DB_NAME="postgres" # 连接到的目标库
PATTERN="test_%"   # 匹配模式

echo "正在查询匹配模式 '$PATTERN' 的数据库..."

# 设置环境变量
# export PGPASSWORD="password"

# 2. 获取所有待删除的数据库列表
# -t: 仅输出元组（不显示表头/脚注）
# -A: 非对齐模式
DBS_TO_DELETE=$(psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -t -A -c "
    SELECT datname FROM pg_database 
    WHERE datname LIKE '$PATTERN' AND datistemplate = false;"
)

if [ -z "$DBS_TO_DELETE" ]; then
    echo "未发现匹配的数据库，退出。"
    exit 0
fi

echo "发现以下数据库待删除:"
echo "$DBS_TO_DELETE"
read -p "确定要删除吗？(y/n) " CONFIRM

if [[ $CONFIRM != "y" ]]; then
    echo "操作取消。"
    exit 1
fi

# 3. 循环删除
for DB in $DBS_TO_DELETE; do
    echo "------------------------------------------"
    echo "正在处理数据库: $DB"

    # 强制断开其他连接 (可选，PostgreSQL 13+ 可选，旧版本必须)
    psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "
        SELECT pg_terminate_backend(pid) 
        FROM pg_stat_activity 
        WHERE datname = '$DB' AND pid <> pg_backend_pid();" > /dev/null

    # 执行删除操作
    # 注意：psql 默认不开启显式事务，所以 DROP DATABASE 可以直接执行
    psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "DROP DATABASE \"$DB\";"

    if [ $? -eq 0 ]; then
        echo "✅ 成功删除 $DB"
    else
        echo "❌ 删除 $DB 失败"
    fi
done

echo "------------------------------------------"
echo "所有清理工作已完成。"