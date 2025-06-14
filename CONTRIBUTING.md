# 貢獻指南

感謝您對 r-fubon-neo P.O.C 專案的興趣！

> **⚠️ P.O.C 專案**: 本專案為概念驗證階段，貢獻指南針對學習和研究目的設計。  
> **專案開發者**: Steve Lo (info@sd.idv.tw)

## 🎯 專案目標

r-fubon-neo 是一個 **概念驗證 (Proof of Concept)** 專案，主要目標包括：

1. **技術探索**: 驗證 Rust 在金融 API 開發的可行性
2. **性能研究**: 展示 Rust vs Python 在金融應用的性能差異
3. **架構設計**: 探索異步、類型安全的金融 SDK 設計模式
4. **教育資源**: 為學習 Rust 金融應用開發提供完整範例

## 🤝 歡迎的貢獻類型

### ✅ 鼓勵的貢獻

- 📚 **文檔改進**: 修正錯誤、改善說明、添加範例
- 🧪 **測試用例**: 添加單元測試、集成測試
- 🔧 **程式碼品質**: 改善程式碼結構、性能優化、錯誤處理
- 💡 **概念驗證**: 新功能的原型實現
- 🐛 **問題修復**: Bug 回報和修復
- 📊 **基準測試**: 性能測試和比較
- 🌐 **國際化**: 多語言支援
- 🎨 **範例程式碼**: 新的使用範例和教學

### ❌ 不適合的貢獻

- ❌ **生產級功能**: 企業級功能實現
- ❌ **商業化**: 商業用途的功能
- ❌ **真實交易**: 涉及實際金錢交易的功能
- ❌ **複雜架構**: 過於複雜的設計模式
- ❌ **重大 API 變更**: 破壞性的 API 修改

## 🚀 如何開始貢獻

### 1. 設置開發環境

```bash
# Fork 並 clone 專案
git clone https://github.com/YOUR_USERNAME/r-fubon-neo.git
cd r-fubon-neo

# 設置開發環境
cargo build
cargo test
```

### 2. 選擇貢獻類型

#### 📚 文檔貢獻
```bash
# 編輯文檔檔案
vim docs/API.md
vim README.md
vim examples/README.md

# 檢查 markdown 格式
# 提交更改
```

#### 🧪 測試貢獻
```bash
# 添加測試
vim src/sdk.rs  # 添加 #[cfg(test)] 模組
vim tests/integration_tests.rs

# 運行測試
cargo test
```

#### 🔧 程式碼貢獻
```bash
# 創建功能分支
git checkout -b feature/improve-error-handling

# 實現改進
vim src/error.rs

# 測試和檢查
cargo test
cargo clippy
cargo fmt
```

## 📋 貢獻流程

### 1. 創建 Issue

在開始貢獻之前，請先創建或檢查相關的 Issue：

- 🐛 **Bug 報告**: 詳細描述問題和重現步驟
- 💡 **功能建議**: 說明新功能的目的和用例
- 📚 **文檔改進**: 指出需要改進的文檔部分
- ❓ **問題討論**: 技術問題或設計討論

### 2. Fork 和分支

```bash
# Fork 專案到您的 GitHub 帳戶
# Clone 到本地
git clone https://github.com/YOUR_USERNAME/r-fubon-neo.git

# 創建功能分支
git checkout -b feature/your-feature-name
# 或
git checkout -b fix/bug-description
# 或
git checkout -b docs/documentation-improvement
```

### 3. 實現更改

#### 程式碼標準

- 🦀 **Rust 慣例**: 遵循 Rust 官方程式碼風格
- 📝 **文檔**: 為公共 API 添加文檔註釋
- 🧪 **測試**: 為新功能添加相應測試
- 🚨 **錯誤處理**: 使用 `Result` 類型進行錯誤處理

```rust
/// 函數功能的簡要描述
///
/// # Arguments
/// * `param` - 參數描述
///
/// # Returns
/// 返回值描述
///
/// # Errors
/// 可能的錯誤情況
///
/// # Examples
/// ```
/// use r_fubon_neo::*;
/// // 使用範例
/// ```
pub fn your_function(param: Type) -> Result<ReturnType> {
    // 實現
}
```

#### 測試要求

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_success() {
        // 成功案例測試
    }

    #[test]
    fn test_function_error() {
        // 錯誤案例測試
    }

    #[tokio::test]
    async fn test_async_function() {
        // 異步函數測試
    }
}
```

### 4. 提交和推送

```bash
# 檢查程式碼品質
cargo fmt
cargo clippy
cargo test

# 提交更改
git add .
git commit -m "feat: add new feature description

詳細說明更改內容和原因

Closes #issue_number"

# 推送到您的 fork
git push origin feature/your-feature-name
```

#### 提交消息格式

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**類型:**
- `feat`: 新功能
- `fix`: 錯誤修復
- `docs`: 文檔更新
- `style`: 程式碼格式
- `refactor`: 重構
- `test`: 測試相關
- `chore`: 建構或輔助工具

**範例:**
```
feat(market_data): add WebSocket reconnection logic

實現自動重連機制，當 WebSocket 連接斷開時
使用指數退避算法進行重連

Closes #123
```

### 5. 創建 Pull Request

1. 前往 GitHub 上的原始專案
2. 點擊 "New Pull Request"
3. 選擇您的分支
4. 填寫 PR 模板

#### PR 模板

```markdown
## 更改描述
簡要描述這個 PR 的目的和更改內容

## 更改類型
- [ ] 🐛 Bug 修復
- [ ] ✨ 新功能
- [ ] 📚 文檔更新
- [ ] 🧪 測試改進
- [ ] 🔧 程式碼品質改進
- [ ] 📊 性能優化

## 測試
- [ ] 已添加新測試
- [ ] 所有測試通過
- [ ] 手動測試完成

## 檢查清單
- [ ] 程式碼通過 `cargo clippy`
- [ ] 程式碼已格式化 (`cargo fmt`)
- [ ] 添加了適當的文檔
- [ ] 遵循 P.O.C 專案目標

## 相關 Issue
Closes #issue_number
```

## 🔍 程式碼審查

### 審查標準

- ✅ **功能正確性**: 程式碼是否實現了預期功能
- ✅ **程式碼品質**: 是否遵循 Rust 最佳實踐
- ✅ **測試覆蓋**: 是否有足夠的測試
- ✅ **文檔完整**: 是否有清晰的文檔
- ✅ **P.O.C 符合性**: 是否符合專案目標

### 審查流程

1. **自動檢查**: CI 會自動運行測試和檢查
2. **人工審查**: 維護者會審查程式碼
3. **反饋處理**: 根據反饋進行修改
4. **合併**: 通過審查後合併到主分支

## 🎯 特殊貢獻指導

### 性能優化

```rust
// 使用 criterion 進行基準測試
#[cfg(test)]
mod benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn benchmark_function(c: &mut Criterion) {
        c.bench_function("function_name", |b| {
            b.iter(|| {
                // 測試程式碼
            })
        });
    }
    
    criterion_group!(benches, benchmark_function);
    criterion_main!(benches);
}
```

### 範例程式碼

新增範例時請包含：

```rust
/*!
# 範例標題 (P.O.C)

⚠️ **P.O.C 專案**: 本範例僅供概念驗證和學習使用
👨‍💻 **開發者**: Steve Lo (info@sd.idv.tw)

## 功能描述
## 使用方法
## ⚠️ 重要提醒
*/

// 完整的範例程式碼
// 包含錯誤處理
// 包含文檔註釋
// 包含測試
```

## 📞 獲得幫助

如果您在貢獻過程中遇到問題：

1. **檢查文檔**: 查看 [開發者指南](./docs/DEVELOPMENT.md)
2. **搜尋 Issues**: 查看是否有類似問題
3. **創建 Issue**: 描述您遇到的問題
4. **聯繫維護者**: Steve Lo (info@sd.idv.tw)
5. **GitHub Discussions**: 參與討論

## 🏆 貢獻者認可

所有貢獻者都會在 README.md 中得到認可。主要貢獻者可能會被邀請成為專案協作者。

### 貢獻者類型

- 🏆 **核心貢獻者**: 長期貢獻且符合專案目標
- 📚 **文檔貢獻者**: 專注於文檔改進
- 🧪 **測試貢獻者**: 專注於測試和品質保證
- 🐛 **問題報告者**: 發現和報告 bug
- 💡 **想法貢獻者**: 提供有價值的建議和想法

## 📄 授權

通過貢獻程式碼，您同意您的貢獻將在與專案相同的授權條款下發布 (MIT OR Apache-2.0)。

## 🙏 致謝

感謝每一位對 r-fubon-neo P.O.C 專案做出貢獻的人！您的貢獻幫助我們探索 Rust 在金融科技領域的應用潛力。

---

**開發者**: Steve Lo (info@sd.idv.tw)  
**專案性質**: P.O.C (Proof of Concept)  
**目標**: 探索 Rust 在金融 API 開發的應用潛力