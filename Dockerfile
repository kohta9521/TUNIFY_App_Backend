# 1. ビルドステージ
FROM rust:1.87-slim-bookworm AS builder

WORKDIR /app

# 依存だけ先に解決してキャッシュを効かせる
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# 実際のソースをコピー
COPY src ./src

# 本ビルド
RUN cargo build --release && \
    rm -rf /app/target/release/deps /app/target/release/build

# 2. ランタイムステージ（軽量イメージ）
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# 必要なランタイムライブラリのみインストール
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/tunify-backend /usr/local/bin/tunify-backend

# 実行権限を付与
RUN chmod +x /usr/local/bin/tunify-backend

# 環境変数の設定
ENV RUST_LOG=tunify-backend=info,tower_http=info

# ポート8085を公開
EXPOSE 8085

# アプリケーションを起動（ログを確実に表示するため）
CMD ["/usr/local/bin/tunify-backend"]
