using System;
using System.Windows;
using System.Windows.Media;

namespace I2CS.UI
{
    public partial class App : Application
    {
        protected override void OnStartup(StartupEventArgs e)
        {
            try
            {
                base.OnStartup(e);
            }
            catch (Exception ex)
            {
                MessageBox.Show($"启动失败:\n{ex.Message}\n\n堆栈:\n{ex.StackTrace}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                Shutdown(1);
            }
        }
    }
}
