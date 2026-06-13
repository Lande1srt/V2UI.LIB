using System;
using System.Windows;
using System.Windows.Media;

namespace I2CS.UI
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
        }

        private void SwitchToLightTheme(object sender, RoutedEventArgs e)
        {
            ChangeTheme("Themes/LightTheme.xaml");
        }

        private void SwitchToDarkTheme(object sender, RoutedEventArgs e)
        {
            ChangeTheme("Themes/DarkTheme.xaml");
        }

        private void ChangeTheme(string themeUri)
        {
            var newTheme = new ResourceDictionary { Source = new Uri(themeUri, UriKind.Relative) };

            Application.Current.Resources.MergedDictionaries.Clear();
            Application.Current.Resources.MergedDictionaries.Add(newTheme);

            this.Background = (Brush)Application.Current.Resources["BodyBackgroundGradient"];
        }
    }
}
