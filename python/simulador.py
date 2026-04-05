from PyQt5 import QtWidgets
from PyQt5.QtGui import QIntValidator, QDoubleValidator
from PyQt5.QtCore import QThread, pyqtSignal
import sys

from graficos import graficar_convergencia
from interfaz_simulador import Ui_MainWindow
from pso import Simulacion as SimulacionPSO
from ea import Simulacion as SimulacionEA


class GraphWorker(QThread):
    terminado = pyqtSignal(str)

    def __init__(self, fn):
        super().__init__()
        self.fn = fn

    def run(self):
        save_path = self.fn()
        self.terminado.emit(save_path or "")


class MainWindow(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)

        self.ui.graph_pso.setEnabled(False)
        self.ui.graph_ea.setEnabled(False)

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
        self.ui.max_gen_pso.textChanged.connect(self.validar_condicion_termino_pso)
        self.ui.max_gen_ea.textChanged.connect(self.validar_condicion_termino_ea)


    def validar_condicion_termino_pso(self):
        tiene_texto = bool(self.ui.max_gen_pso.text().strip())
        self.ui.graph_pso.setEnabled(tiene_texto)

        
    def validar_condicion_termino_ea(self):
        tiene_texto = bool(self.ui.max_gen_ea.text().strip())
        self.ui.graph_ea.setEnabled(tiene_texto)

    
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
        self.ui.exec_ea.setDisabled(lock)
        if self.ui.max_gen_pso.text(): self.ui.graph_pso.setDisabled(lock)
        if self.ui.max_gen_ea.text(): self.ui.graph_ea.setDisabled(lock)


    def ejecutar_pso(self):
        self.lock_buttons()
        params = self.leer_params_pso()
        self.indicar_ejecucion("Ejecutando simulación gráfica del algoritmo PSO...")

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

    
    def ejecutar_ea(self):
        self.lock_buttons()
        params = self.leer_params_ea()
        self.indicar_ejecucion("Ejecutando simulación gráfica del algoritmo genético...")

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


    def graficar_pso(self):
        params = self.leer_params_pso()

        def simulaciones():
            historiales = []

            for _ in range(params["iter"]):
                sim = SimulacionPSO(
                    max_iter = params["max_iter"],
                    n = params["n"],
                    c1 = params["c1"],
                    c2 = params["c2"],
                    w = params["w"],
                    vel = params["vel"],
                    headless = True
                )
                historiales.append(sim.run_headless())

            return graficar_convergencia(historiales, "Convergencia PSO", "Iteración", "PSO")

        self.indicar_ejecucion("Generando gráficos de convergencia para el algoritmo PSO...")
        self.worker_graph_pso = GraphWorker(simulaciones)
        self.worker_graph_pso.terminado.connect(self.on_grafico_listo)
        self.lock_buttons()
        self.worker_graph_pso.start()


    def graficar_ea(self):
        params = self.leer_params_ea()

        def simulaciones():
            historiales = []

            for _ in range(params["iter"]):
                sim = SimulacionEA(
                    max_gen = params["max_gen"],
                    n = params["n"],
                    k = params["k"],
                    pc = params["pc"],
                    pm = params["pm"],
                    reinsercion = params["reinsercion"],
                    headless = True
                )
                historiales.append(sim.run_headless())
            
            return graficar_convergencia(historiales, "Convergencia Algoritmo Genético", "Generación", "EA")

        self.indicar_ejecucion("Generando gráficos de convergencia del algoritmo genético...")
        self.worker_graph_ea = GraphWorker(simulaciones)
        self.worker_graph_ea.terminado.connect(self.on_grafico_listo)
        self.lock_buttons()
        self.worker_graph_ea.start()


    def indicar_ejecucion(self, mensaje):
        self.ui.statusBar.setStyleSheet('QStatusBar {background: transparent; font: 75 italic 9pt "Arial"; color: #a500f8;}')
        self.ui.statusBar.showMessage(mensaje, 5000)


    def on_grafico_listo(self, save_path: str):
        self.lock_buttons(lock = False)

        mensaje = f"Gráfico de convergencia generado con éxito en [ {save_path} ]"
        self.ui.statusBar.setStyleSheet('QStatusBar {background: transparent; font: 75 italic 9pt "Arial"; color: rgb(98, 195, 0);}')
        self.ui.statusBar.showMessage(mensaje, 5000)


if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    ventana = MainWindow()
    ventana.show()
    sys.exit(app.exec_())