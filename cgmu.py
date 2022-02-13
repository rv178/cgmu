from PyQt5 import Qt
import os
import sys
# -------------------------------

if not os.geteuid() == 0:
    sys.exit("\nYou need to be a root user to run this script!\n")

# -------------------------------

class MainWindow(Qt.QMainWindow):

    def __init__(self):
        super(MainWindow, self).__init__()

        self.resize(600, 500)
        self.setWindowTitle("CGMU")

        self.setStyleSheet("background-color : #242426; color: white")
        self.main_widget = Qt.QWidget()
        self.setCentralWidget(self.main_widget)
        layout = Qt.QVBoxLayout(self.main_widget)

        self.comboBox = Qt.QComboBox(self)

        self.comboBox.addItem("Performance")
        self.comboBox.addItem("Ondemand")
        self.comboBox.addItem("Shedutil")
        self.comboBox.addItem("Powersave")
        self.comboBox.activated[str].connect(self.combo_text)
        self.text = ""

        self.btn = Qt.QPushButton("Save Changes", self)
        self.btn.clicked.connect(self.change_cpu_g)

        layout.addWidget(self.comboBox)
        layout.addWidget(self.btn)
        layout.setAlignment(Qt.Qt.AlignCenter)

    def change_cpu_g(self, text):
        currentWord = str(self.text)

        if currentWord == "Performance":
            os.system("echo 'performance' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")

        elif currentWord == "Ondemand":
            os.system("echo 'ondemand' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")

        elif currentWord == "Shedutil":
            os.system("echo 'schedutil' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")

        elif currentWord == "Powersave":
            os.system("echo 'powersave' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")
            
    def combo_text(self, text):
        self.text = self.comboBox.currentText()

# -------------------------------

if __name__ == '__main__':
    import sys
    app = Qt.QApplication(sys.argv)
    ex = MainWindow()
    ex.show()
    sys.exit(app.exec_())
