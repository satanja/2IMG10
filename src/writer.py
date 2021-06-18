import os
import tifffile as tiff
import numpy as np


def write_layer(layer, frame: int, path: str):
    id = format(frame + 1, '03')
    with open(f'{path}/layer-{id}.txt', 'w') as file:
        max = layer.max()
        min = layer.min()
        file.write(f'1600 160 1 1 {min} {max}\n')

        count = 0
        for i in range(len(layer)):
            row = layer[i]
            for j in range(len(row)):
                count += 1
                if j == 0:
                    file.write(f'{layer[i][j]}')
                else:
                    file.write(f' {layer[i][j]}')
            file.write('\n')
        print(count)
        file.write('\n')

def write_layers(path: str):
    detrended = tiff.imread('../data/detrended.tiff')
    for i in range(len(detrended)):
        write_layer(detrended[i], i, path)

write_layers(".")