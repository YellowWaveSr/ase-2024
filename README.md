The MATLAB file and the test waves are in folder target/

Parameter: modulation frequency = 5.0; vibration width = 0.01; amplitude = 1.0
(The outputs are attached to this folder.)
Test 1: sweep.wav
Subtract the outputs directly would cause some error. Shown below.
>> plot(data_matlab(1:end,1) - data_rust(1:end,1))

![image](https://github.com/YellowWaveSr/ase-2024/assets/43238578/1f30ac90-915a-4a1b-80b2-8490ca905c54)

However, on the second inspection, I realized that this is due to the difference in the subscript.
Subtract the index by 1 solves the problem.
>> plot(data_matlab(1:end-1,1) - data_rust(2:end,1))

![image](https://github.com/YellowWaveSr/ase-2024/assets/43238578/c7e4864e-ec5b-43d4-ae5d-f6303c54fac6)

Test 2: stereo_chord.wav. The results are similar.
>> plot(data_matlab(1:end,1) - data_rust(1:end,1))

![image](https://github.com/YellowWaveSr/ase-2024/assets/43238578/3fa029e8-b9da-4bf2-b477-0c2a58dc131e)

After changing the index:

![image](https://github.com/YellowWaveSr/ase-2024/assets/43238578/f3bd1d74-fb34-49a5-b8f5-41254a5f0cc2)

Spoiler: the matlab code can only handle one channel. My code can handle both. 
