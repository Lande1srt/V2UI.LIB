# V2UI.LIB

个人程序自用风格的UI项目，收录多个设计风格，包含CSS样式表与C# WPF样式字典，也包含主题切换演示应用程序，提供浅色和深色两种现代化简约主题。

## 特性

- 🌓 双主题支持 - 浅色主题和深色主题
- 🎨 现代化UI设计 - 圆角、渐变、动画效果
- 📦 丰富的控件样式 - 按钮、标签、下拉框、开关、单选按钮、复选框、进度条、滑块、文本框、选项卡、折叠面板等
- 🎯 完全自定义的控件模板 - 不依赖默认WPF样式

## 支持的控件

| 控件类型     | 样式名称                 | 说明         |
| ------------ | ------------------------ | ------------ |
| Button       | PrimaryButtonStyle       | 主要操作按钮 |
| Button       | SecondaryButtonStyle     | 次要操作按钮 |
| Button       | MiniButtonStyle          | 小尺寸按钮   |
| Border       | CardBorderStyle          | 内容卡片     |
| TextBlock    | HeadingTextBlockStyle    | 标题文本     |
| TextBlock    | BodyTextBlockStyle       | 正文文本     |
| TextBlock    | CardHeadingStyle         | 卡片标题     |
| Border       | TagBorderStyle           | 标签容器     |
| TextBlock    | TagTextStyle             | 标签文本     |
| ComboBox     | PrimaryComboBoxStyle     | 下拉选择框   |
| ToggleButton | PrimaryToggleButtonStyle | 开关按钮     |
| RadioButton  | PrimaryRadioButtonStyle  | 单选按钮     |
| CheckBox     | PrimaryCheckBoxStyle     | 复选框       |
| ProgressBar  | PrimaryProgressBarStyle  | 进度条       |
| Slider       | PrimarySliderStyle       | 滑块控件     |
| TextBox      | PrimaryTextBoxStyle      | 文本输入框   |
| TabControl   | PrimaryTabControlStyle   | 选项卡控件   |
| TabItem      | PrimaryTabItemStyle      | 单个选项卡   |
| Expander     | PrimaryExpanderStyle     | 折叠面板     |
| ScrollBar    | (无)                     | 自动应用     |

## 自定义主题

要创建自定义主题，可以复制现有主题文件并修改颜色定义：

```xml
<!-- 示例颜色定义 -->
<SolidColorBrush x:Key="BodyBackground" Color="#YourColor" />
<LinearGradientBrush x:Key="BodyBackgroundGradient" StartPoint="0,0" EndPoint="0,1">
    <GradientStop Color="#Color1" Offset="0" />
    <GradientStop Color="#Color2" Offset="1" />
</LinearGradientBrush>
```

## 技术栈

- .NET Framework 4.8
- WPF (Windows Presentation Foundation)
- XAML
- C#
- CSS
- HTML

## 许可证

本项目使用 Apache License 2.0 - 详见 [LICENSE](LICENSE) 文件

**享受精美的WPF主题体验！** 🎉
