import pygame

from PSOParticle import Particle
from InformationTab import InformationTab
from Rastrigin import Rastrigin

class Simulacion:
    '''
    Simulación del algoritmo Particle Swarm Optimization
    '''
    
    def __init__(self,
        max_iter: int = None,
        n: int = 200, 
        c1: float = 3.0, 
        c2: float = 10.0, 
        w: float = 100.0, 
        vel: float = 4.0
    ):
        pygame.init()

        self.width = 900
        self.height = 900

        self.screen = pygame.display.set_mode((self.width, self.height))
        pygame.display.set_caption("PSO")
        #self.bg = pygame.image.load("assets/circulos.png").convert()
        rastrigin_map = Rastrigin(self.width, self.height)
        self.bg = rastrigin_map.generate()
        self.gbest_img = pygame.image.load("assets/global_best.png").convert_alpha()

        self.max_iter = max_iter

        # Parámetros de las partículas
        self.n = n
        self.learning_c1 = c1
        self.learning_c2 = c2
        self.inercia = w
        self.max_velocity = vel

        self.gbest = (0, 0)
        self.gbest_fitness = -999999

        # Generación de partículas
        self.particles = [
            Particle(self.screen) for _ in range(self.n)
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
        self.iteracion = 0

    # Main loop
    def run(self):
        while self.running:
            self.handle_events()
            if self.reproduciendo and not self.stop: self.update()
            self.render()
            self.clock.tick(20)

        pygame.quit()


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

        # Evaluar el fitness
        for particle in self.particles:
            fitness = particle.evaluate()

            if fitness > self.gbest_fitness:
                self.gbest_fitness = fitness
                self.gbest = (particle.x, particle.y)

        # Mover las partículas
        for particle in self.particles:
            particle.move(self.screen, self.gbest,
                c1 = self.learning_c1,
                c2 = self.learning_c2,
                w = self.inercia,
                max_vel = self.max_velocity
            )

        self.iteracion += 1

        if self.max_iter is not None and self.iteracion >= self.max_iter:
            self.stop = True


    # Renderizar assets y objetos
    def render(self):
        self.screen.blit(self.bg, (0, 0))  # imagen de fondo

        # Dibujar partículas
        for particle in self.particles:
            particle.draw(self.screen)

        # Dibujar el global best
        self.screen.blit(self.gbest_img, dest = (int(self.gbest[0]), int(self.gbest[1])))

        estado_str = "Reproduciendo" if self.reproduciendo else "Pausado"        

        # Dibujar información
        self.information.render(self.screen, 
            {
                "Estado": estado_str if not self.stop else "Máximo de iteraciones alcanzado",
                "Iteración": self.iteracion,
                "Cantidad de partículas": len(self.particles),
                "Mejor posición global": f"({self.gbest[0]:0.2f}, {self.gbest[1]:0.2f})",
                "Fitness global": self.gbest_fitness,
                "Learning factor C1": self.learning_c1,
                "Learning factor C2": self.learning_c2,
                "Inercia W": self.inercia,
                "Velocidad máxima": self.max_velocity
            }
        )

        self.screen.blit(self.tutorial, (10, self.height - self.tutorial.height))

        pygame.display.flip()


if __name__ == "__main__":
    sim = Simulacion()
    sim.run()