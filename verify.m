clear;
clc;
close all;

% Load the wave file
wave_file_path = 'sweep.wav';
[wave_data, sample_rate] = audioread(wave_file_path);

% Load the text file
text_file_path = 'output.txt';
text_data = load(text_file_path);

% Compute the absolute difference between wave and text data
difference = abs(wave_data(:, 1) - text_data(:, 1));

% Display the maximum absolute difference
disp(['Maximum absolute difference: ' num2str(max(difference))]);
