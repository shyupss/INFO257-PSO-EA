from PyQt5 import QtWidgets
from PyQt5.QtGui import QIntValidator, QDoubleValidator
import sys

from interfaz_simulador import Ui_MainWindow
from pso import Simulacion as SimulacionPSO
from ea import Simulacion as SimulacionEA


class MainWindow(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)
        self.conectar_señales()

        self.ui.ea_n.setValidator(QIntValidator(1,99999999))
        self.ui.input_k.setValidator(QIntValidator(1,99999999))
        self.ui.input_pc.setValidator(QDoubleValidator(0, 99999999, 10))
        self.ui.input_pm.setValidator(QDoubleValidator(0, 99999999, 10))
        self.ui.max_gen_ea.setValidator(QIntValidator(1,99999999))
        self.ui.iter_ea.setValidator(QIntValidator(1,99999999))
        
        self.ui.pso_n.setValidator(QIntValidator(1,99999999))
        self.ui.input_c1.setValidator(QDoubleValidator(0, 99999999, 10))
        self.ui.input_c2.setValidator(QDoubleValidator(0, 99999999, 10))
        self.ui.input_w.setValidator(QDoubleValidator(0, 99999999, 10))
        self.ui.input_vel.setValidator(QDoubleValidator(0, 99999999, 10))
        self.ui.max_gen_pso.setValidator(QIntValidator(1,99999999))
        self.ui.iter_pso.setValidator(QIntValidator(1,99999999))
        


    def conectar_señales(self):
        self.ui.exec_pso.clicked.connect(self.ejecutar_pso)
        self.ui.graph_pso.clicked.connect(self.graficar_pso)
        self.ui.exec_ea.clicked.connect(self.ejecutar_ea)
        self.ui.graph_ea.clicked.connect(self.graficar_ea)


    def leer_params_pso(self):
        def val(widget, default):
            texto = widget.text().strip()
            return float(texto) if texto else default

        return {
            "max_iter": val(self.ui.max_gen_pso, None),
            "iter": int(val(self.ui.iter_pso, 1)),
            "n": int(val(self.ui.pso_n, 50)),
            "c1": val(self.ui.input_c1, 3.0),
            "c2": val(self.ui.input_c2, 10.0),
            "w": val(self.ui.input_w, 100.0),
            "vel": val(self.ui.input_vel, 4.0),
        }


    def leer_params_ea(self):
        def val(widget, default):
            texto = widget.text().strip()
            return float(texto) if texto else default
        
        return {
            "max_gen": val(self.ui.max_gen_ea, None),
            "iter": int(val(self.ui.iter_ea, 1)),
            "n": int(val(self.ui.ea_n, 50)),
            "k": int(val(self.ui.input_k, 3)),
            "pc": val(self.ui.input_pc, 0.8),
            "pm": val(self.ui.input_pm, 0.03),
            "reinsercion": 1 if self.ui.input_reinsercion.currentText() == "Generacional" else 2
        }
    

    def lock_buttons(self, lock = True):
        self.ui.exec_pso.setDisabled(lock)
        self.ui.graph_pso.setDisabled(lock)
        self.ui.exec_ea.setDisabled(lock)
        self.ui.graph_ea.setDisabled(lock)


    def ejecutar_pso(self):
        self.lock_buttons()
        params = self.leer_params_pso()

        sim = SimulacionPSO(
            max_iter = params["max_iter"],
            n = params["n"],
            c1 = params["c1"],
            c2 = params["c2"],
            w = params["w"],
            vel = params["vel"]
        )
        sim.run()
        self.lock_buttons(lock = False)


    def graficar_pso(self):
        params = self.leer_params_pso()

    
    def ejecutar_ea(self):
        self.lock_buttons()
        params = self.leer_params_ea()

        sim = SimulacionEA(
            max_gen = params["max_gen"],
            n = params["n"],
            k = params["k"],
            pc = params["pc"],
            pm = params["pm"],
            reinsercion = params["reinsercion"]
        )
        sim.run()
        self.lock_buttons(lock = False)


    def graficar_ea(self):
        params = self.leer_params_ea()


if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    ventana = MainWindow()
    ventana.show()
    sys.exit(app.exec_())