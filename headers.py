import os

def combine_headers(input_folder, output_file):
    with open(output_file, 'w') as output:
        for filename in os.listdir(input_folder):
            if filename.endswith('.h'):
                header_path = os.path.join(input_folder, filename)
                with open(header_path, 'r') as header:
                    output.write(header.read())
                    output.write('\n')

if __name__ == "__main__":
    # Replace 'input_folder' with the path to the folder containing your header files
    input_folder = 'cubiomes'

    # Replace 'output_file.h' with the desired name for the combined header file
    output_file = 'cubiomes.h'

    combine_headers(input_folder, output_file)
    print(f"Combined header files into {output_file}")
