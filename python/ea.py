import pygame
import random

from InformationTab import InformationTab
from Rastrigin import Rastrigin
from EAParticle import Particle, LONGITUD_CROMOSOMA

class Simulacion:
    '''
    Simulación del algoritmo evolutivo "Algoritmo genético"
    '''

    def __init__(self,
        max_gen: int = None,
        n: int = 200, 
        k: int = 3, 
        pc: float = 0.8, 
        pm: float = 0.03, 
        reinsercion: int = 1,
        headless: bool = False
    ):
        pygame.init()

        # Parámetros del algoritmo genético
        self.N = n                              # Tamaño de población
        self.K = k                              # Tamaño del torneo
        self.Pc = pc                            # Probabilidad de cruzamiento
        self.Pm = pm                            # Probabilidad de mutación
        self.modo_reinsercion = reinsercion     # 1 = generacional, 2 = steady state

        self.width = 900
        self.height = 900

        # Inicialización con o sin motor gráfico
        if headless:
            self.screen = pygame.Surface((self.width, self.height))
        else:
            self.screen = pygame.display.set_mode((self.width, self.height))
            pygame.display.set_caption("Algoritmo Genético")
            self.gbest_img = pygame.image.load("assets/global_best.png").convert_alpha()

        rastrigin_map = Rastrigin(self.width, self.height)
        self.bg = rastrigin_map.generate()

        self.max_gen = max_gen

        # Parámetros de las partículas
        self.gbest_cromosoma = None
        self.gbest_fitness = -999999

        # Generación de partículas
        self.particles = [
            Particle(self.screen) for _ in range(self.N)
        ]

        # Panel de información
        self.information = InformationTab()
        self.tutorial = self.information.font.render(
            "[Barra espaciadora] para reproducir, presiona cualquier tecla para avanzar una iteración.",
            True,
            (0, 0, 0)
        )

        self.clock = pygame.time.Clock()
        self.running = True
        self.reproduciendo = False
        self.stop = False
        self.generacion = 0


    # Main loop
    def run(self):
        while self.running:
            self.handle_events()

            if self.reproduciendo and not self.stop:
                self.update()

            self.render()
            self.clock.tick(20)

        pygame.quit()

    
    def run_headless(self):
        """Corre la simulación sin pygame, retorna historial de fitness por generación."""
        historial = []

        max_gen = int(self.max_gen) if self.max_gen is not None else 200

        for _ in range(max_gen):
            if self.modo_reinsercion == 1:
                self._generacional()
            else:
                self._steady_state()

            self.generacion += 1
            historial.append(self.gbest_fitness)

        return historial


    # Procesar los eventos del usuario
    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_SPACE:
                    self.reproduciendo = not self.reproduciendo
                elif not self.stop:
                    self.update()

            if event.type == pygame.QUIT:
                self.running = False


    # Actualizar lógica
    def update(self):
        if self.modo_reinsercion == 1:
            self._generacional()
        else:
            self._steady_state()

        self.generacion += 1

        if self.max_gen is not None and self.generacion >= self.max_gen:
            self.stop = True

    # Selección
    def _torneo(self):
        '''
        Selecciona un individuo mediante torneo de tamaño K.
        Escoge K individuos al azar y retorna el de mayor fitness.
        '''
        participantes = random.sample(self.particles, self.K)
        return max(participantes, key=lambda p: p.fitness)
    

    # Cruzamiento
    def _cruzamiento(self, padre_a: Particle, padre_b: Particle):
        '''
        Cruzamiento de un punto. Retorna dos nuevos Particle con los
        cromosomas resultantes. Si no se aplica cruzamiento, los hijos
        son copias de los padres.
        '''
        hijo1 = Particle(self.screen)
        hijo2 = Particle(self.screen)
 
        if random.random() < self.Pc:
            punto = random.randint(1, LONGITUD_CROMOSOMA - 1)
            hijo1.move(padre_a.cromosoma[:punto] + padre_b.cromosoma[punto:])
            hijo2.move(padre_b.cromosoma[:punto] + padre_a.cromosoma[punto:])
        else:
            hijo1.move(padre_a.cromosoma[:])
            hijo2.move(padre_b.cromosoma[:])
 
        return hijo1, hijo2


    # Mutación
    def _mutar(self, particle: Particle):
        '''
        Mutación bit-flip con probabilidad Pm por bit.
        Con Pm = 1/L se espera mutar exactamente 1 bit en promedio.
        '''
        cromosoma = particle.cromosoma[:]
        for i in range(LONGITUD_CROMOSOMA):
            if random.random() < self.Pm:
                cromosoma[i] = 1 - cromosoma[i]

        particle.move(cromosoma)


    # Reinserción
    def _actualizar_gbest(self):
        for p in self.particles:
            if p.fitness > self.gbest_fitness:
                self.gbest_fitness = p.fitness
                self.gbest_cromosoma = p.cromosoma[:]
                self.gbest = p.decode()


    def _generacional(self):
        '''
        Reemplaza toda la población con hijos nuevos.
        Conserva el mejor individuo actual (elitismo = 1) para que
        el mejor fitness nunca retroceda.
        '''
        # Guardar el élite (mejor de la generación actual)
        elite = max(self.particles, key=lambda p: p.fitness)
 
        nueva_poblacion = []
 
        # Generar N-1 hijos (el slot restante lo ocupa el élite)
        while len(nueva_poblacion) < self.N - 1:
            padre_a = self._torneo()
            padre_b = self._torneo()
 
            hijo1, hijo2 = self._cruzamiento(padre_a, padre_b)
 
            self._mutar(hijo1)
            self._mutar(hijo2)
 
            nueva_poblacion.append(hijo1)
            if len(nueva_poblacion) < self.N - 1:
                nueva_poblacion.append(hijo2)
 
        # Insertar élite sin modificar
        elite_nuevo = Particle(self.screen)
        elite_nuevo.move(elite.cromosoma[:])
        nueva_poblacion.append(elite_nuevo)
 
        self.particles = nueva_poblacion
        self._actualizar_gbest()


    def _steady_state(self):
        '''
        Genera 2 hijos por iteración. Cada hijo reemplaza al peor
        individuo de la población solo si es mejor que él.
        Se realizan N/2 reemplazos para ser comparable al generacional.
        '''
        reemplazos = max(1, self.N // 2)
 
        for _ in range(reemplazos):
            padre_a = self._torneo()
            padre_b = self._torneo()
 
            hijo1, hijo2 = self._cruzamiento(padre_a, padre_b)
            self._mutar(hijo1)
            self._mutar(hijo2)
 
            # Reemplazar al peor si el hijo es mejor
            for hijo in (hijo1, hijo2):
                peor = min(self.particles, key=lambda p: p.fitness)
                if hijo.fitness > peor.fitness:
                    idx = self.particles.index(peor)
                    self.particles[idx] = hijo
 
        self._actualizar_gbest()


    # Renderizar assets y objetos
    def render(self):
        # Dibujar fondo
        self.screen.blit(self.bg, (0, 0))

        # Dibujar partículas
        for particle in self.particles:
            particle.draw(self.screen)

        # Dibujar el global best
        if self.gbest_cromosoma is not None:
            gbest_particle = Particle(self.screen)
            gbest_particle.cromosoma = self.gbest_cromosoma
            gx, gy = gbest_particle.translate_coords_to_screen()
            self.screen.blit(
                self.gbest_img,
                dest= (int(gx) - self.gbest_img.get_width() // 2,
                      int(gy) - self.gbest_img.get_height() // 2)
            )

        gx_dom, gy_dom = self.gbest if self.gbest_cromosoma else (0.0, 0.0)
 
        modo_str = "Generacional" if self.modo_reinsercion == 1 else "Steady State"
        estado_str = "Reproduciendo" if self.reproduciendo else "Pausado"

        # Dibujar información
        self.information.render(self.screen, 
            {
                "Estado": estado_str if not self.stop else "Máximo de generaciones alcanzado",
                "Modo de reinserción": modo_str,
                "Generación": self.generacion,
                "Población (N)": self.N,
                "Torneo (K)": self.K,
                "Probabilidad de cruzamiento (Pc)": f"{self.Pc:.2f}",
                "Probabilidad de mutación (Pm)": f"{self.Pm:.4f}",
                "Mejor posición": f"({gx_dom:.3f}, {gy_dom:.3f})",
                "Mejor fitness": f"{self.gbest_fitness:.4f}",
            }
        )

        self.screen.blit(self.tutorial, (10, self.height - self.tutorial.get_height()))

        pygame.display.flip()


if __name__ == "__main__":
    sim = Simulacion()
    sim.run()