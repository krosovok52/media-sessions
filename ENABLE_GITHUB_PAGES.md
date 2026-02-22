# 🚀 Инструкция по включению GitHub Pages

## Проблема

Документация не обновляется на https://krosovok52.github.io/media-sessions/

## Решение

### Вариант 1: Включить GitHub Pages через Actions (Рекомендуется)

1. Перейдите на GitHub: https://github.com/krosovok52/media-sessions/settings/pages

2. В разделе **Source** выберите:
   - **GitHub Actions** (вместо "Deploy from a branch")

3. Сохраните изменения

4. Перейдите на вкладку **Actions**: https://github.com/krosovok52/media-sessions/actions

5. Найдите workflow **"Deploy Docs to GitHub Pages"**

6. Нажмите **Run workflow** → **Run workflow**

7. Дождитесь завершения (зелёная галочка ✅)

8. Документация будет доступна по адресу:
   ```
   https://krosovok52.github.io/media-sessions/
   ```

### Вариант 2: Использовать ветку gh-pages

Если не хотите использовать Actions:

1. Перейдите на GitHub: https://github.com/krosovok52/media-sessions/settings/pages

2. В разделе **Source** выберите:
   - **Deploy from a branch**

3. Выберите:
   - Branch: **gh-pages**
   - Folder: **/ (root)**

4. Сохраните

5. Запушьте документацию вручную:

```bash
cd c:\Users\rykov\OneDrive\Рабочий стол\MediaSession

# Установите mdbook
cargo install mdbook

# Соберите документацию
cd docs
mdbook build

# Опубликуйте
git checkout -b gh-pages
git rm -rf .
cp -r book/* .
touch .nojekyll
git add .
git commit -m "docs: publish"
git push origin gh-pages --force
git checkout main
```

### Вариант 3: Использовать готовый скрипт

```powershell
# Windows
cd c:\Users\rykov\OneDrive\Рабочий стол\MediaSession
.\quick-publish.bat
```

---

## Проверка

После публикации:

1. Откройте https://github.com/krosovok52/media-sessions/actions
2. Найдите последний запуск workflow
3. Убедитесь, что он завершился успешно (зелёная галочка ✅)
4. Откройте https://krosovok52.github.io/media-sessions/

---

## Troubleshooting

### Workflow не запускается

1. Проверьте что workflow файл существует: `.github/workflows/deploy-simple.yml`
2. Перейдите на вкладку Actions
3. Нажмите на workflow
4. Нажмите **Run workflow**

### 404 ошибка

1. Подождите 2-3 минуты после публикации
2. Проверьте что Pages настроен правильно
3. Очистите кэш браузера

### Ошибка сборки

1. Проверьте логи workflow на GitHub Actions
2. Исправьте ошибки в документации
3. Запушите изменения снова

---

## Контакты

Если проблема не решена:
- GitHub Issues: https://github.com/krosovok52/media-sessions/issues
- Telegram: [@krosov_ok](https://t.me/krosov_ok)
