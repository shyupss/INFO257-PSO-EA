import pygame
import random
import numpy as np
from Rastrigin import Rastrigin


DOMINIO_MIN = -3
DOMINIO_MAX = 7
DOMINIO_SIZE = DOMINIO_MAX - DOMINIO_MIN
 
BITS_POR_VAR = 16
LONGITUD_CROMOSOMA = BITS_POR_VAR * 2


class Particle:
    '''
    Partícula para la simulación de el algoritmo evolutivo "Algoritmo Genético"
    '''

    def __init__(self, screen: pygame.Surface, radius = 4, color = (255, 255, 255)):
        self.color = color
        self.radius = radius

        w, h = screen.get_size()
        self.width = w
        self.height = h

        self.cromosoma = [random.randint(0, 1) for _ in range(LONGITUD_CROMOSOMA)]
 
        self.fitness = None
        self.evaluate()


    def decode(self):
        '''
        Decodifica el cromosoma binario
        '''
        max_val = (2 ** BITS_POR_VAR) - 1
 
        int_x = int(''.join(str(b) for b in self.cromosoma[:BITS_POR_VAR]), 2)
        int_y = int(''.join(str(b) for b in self.cromosoma[BITS_POR_VAR:]), 2)
 
        x = DOMINIO_MIN + (int_x / max_val) * DOMINIO_SIZE
        y = DOMINIO_MIN + (int_y / max_val) * DOMINIO_SIZE
 
        return x, y


    def translate_coords_to_screen(self):
        x, y = self.decode()
    
        px = (x - DOMINIO_MIN) / DOMINIO_SIZE * self.width
        py = (y - DOMINIO_MIN) / DOMINIO_SIZE * self.height

        return px, py

    def draw(self, screen):
        px, py = self.translate_coords_to_screen()

        pygame.draw.circle(
            screen,
            self.color,
            (int(px), int(py)),
            self.radius
        )


    def evaluate(self):
        x, y = self.decode()
        self.fitness = -Rastrigin.evaluate(x, y)

        return self.fitness


    def move(self, cromosoma):
        self.cromosoma = cromosoma
        self.evaluate()