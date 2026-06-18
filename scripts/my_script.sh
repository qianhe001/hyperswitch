
export DB_USER="db_user"
export DB_PASS="db_pass"
export DB_NAME="locker"
sudo -u postgres psql -c "CREATE DATABASE $DB_NAME"
sudo -u postgres psql -c "DROP DATABASE $DB_NAME"

# 1. 设置环境变量（CONTROL_CENTER_URL 可以随意设置，不影响执行）
export HYPERSWITCH_SERVER_URL="http://localhost:8080"
export HYPERSWITCH_CONTROL_CENTER_URL="http://localhost:9000"  # 即使未启动也可以
./scripts/create_default_user.sh


sudo docker compose -f docker-compose-superposition.yml up -d --pull never 
sudo docker compose -f docker-compose-superposition.yml stop hyperswitch-web-sdk
sudo docker compose -f docker-compose-superposition.yml rm hyperswitch-web-sdk
sudo docker compose -f docker-compose-superposition.yml up -d --force-recreate --pull never hyperswitch-control-center
sudo docker compose -f docker-compose-superposition.yml up -d --force-recreate --pull never hyperswitch-web-sdk
sudo docker compose -f docker-compose-superposition.yml up --force-recreate --pull never hyperswitch-web-sdk
sudo docker compose -f docker-compose-superposition.yml up -d

cargo run -j 1

访问sdk: http://192.168.1.69:9050/HyperLoader.js
访问控制中心: http://192.168.1.69:9000/dashboard/home

#lock服务https://github.com/juspay/hyperswitch-card-vault/blob/main/docs/guides/setup.md


编译controller-center
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
sudo apt update
sudo apt install -y clang llvm lld pkg-config build-essential
wasm-pack build \
  --target web \
  --out-dir ./hyperswitch-control-center/public/hyperswitch/wasm \
  --out-name euclid \
  ./crates/euclid_wasm \
  -- --features dummy_connector,v1

cd hyperswitch-control-center
npm install
npm run re:build