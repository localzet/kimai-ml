# Docker сборка - Решение проблем

## Проблема: Cargo.lock not found

### Решение 1: Использовать оптимизированный Dockerfile (рекомендуется)

```bash
docker build -f Dockerfile.optimized -t kimai-ml-rust:latest .
```

Этот Dockerfile не требует Cargo.lock и работает сразу.

### Решение 2: Сгенерировать Cargo.lock локально

```bash
cd ai-ml-rust
cargo build  # Это создаст Cargo.lock
git add Cargo.lock
git commit -m "Add Cargo.lock"
```

Затем обычная сборка:
```bash
docker build -t kimai-ml-rust:latest .
```

### Решение 3: Обновить .dockerignore

Убедитесь, что `Cargo.lock` НЕ в `.dockerignore`:

```dockerignore
target/
.git/
# Cargo.lock НЕ игнорируем!
```

## Рекомендация

Используйте `Dockerfile.optimized` - он проще и не требует Cargo.lock для работы.

CI/CD уже настроен на использование `Dockerfile.optimized`.

