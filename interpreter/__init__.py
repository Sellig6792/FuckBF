class Interpreter:
    def __init__(self,
                 code: str,
                 pointer: int = 0,

                 stdin: str = None,
                 stdout: str = '',

                 stack: list = None,
                 cells: list[int] = None,

                 function_cells: list[str] = None,

                 is_function: bool = False,
                 ):
        self.is_function = is_function
        self.i = 0
        self.code = code.replace('\n', '').replace('\t', '').replace(' ', '')
        if ',' in self.code and not stdin:
            stdin = [*input()]

        self.stack = stack or []
        self.cells = cells or [0] * 30000

        self.function_cells = function_cells or [''] * 30000

        self.stdin = stdin
        self.stdout = stdout

        self.pointer = pointer

        self.writing_function: bool = False
        self.in_comment: bool = False

    @property
    def cell(self):
        return self.cells[self.pointer]

    @cell.setter
    def cell(self, value):
        self.cells[self.pointer] = value

    @property
    def function_cell(self):
        return self.function_cells[self.pointer]

    @function_cell.setter
    def function_cell(self, value):
        self.function_cells[self.pointer] = value

    def __call__(self, *args, **kwargs) -> dict:
        while self.i < len(self.code):
            instruction = self.code[self.i]

            if instruction == '#':
                self.i += 1
                self.in_comment = not self.in_comment
                continue
            if self.in_comment:
                self.i += 1
                continue

            if self.writing_function and instruction != ';':
                self.function_cell += instruction
                self.i += 1
                continue

            if instruction == '>':
                self.right()
            if instruction == '<':
                self.left()

            if instruction == '+':
                self.add()
            if instruction == '-':
                self.sub()

            if instruction == '.':
                self.outp()
            if instruction == ',':
                self.inp()

            if instruction == '[':
                self.start_loop()
            if instruction == ']':
                self.end_loop()

            if instruction == ':':
                self.start_function()
            if instruction == ';':
                self.end_function()
            if instruction == 'x':
                self.execute()
            self.i += 1

        if self.is_function:
            return {
                'stdin': self.stdin,
                'stdout': self.stdout,
                'cells': self.cells,
                'function_cells': self.function_cells,
                'stack': self.stack,
                'pointer': self.pointer
            }
        else:
            print(self.stdout)

    def right(self):
        """
        Decrement the data pointer (to point to the next cell to the left).
        """
        self.pointer = (self.pointer + 1) % 30000

    def left(self):
        """
        Decrement the data pointer (to point to the next cell to the left).
        """
        self.pointer = (self.pointer - 1) % 30000

    def add(self):
        """
        Increment (increase by one) the byte at the data pointer.
        """
        self.cell = (self.cell + 1) % 255

    def sub(self):
        """
        Decrement (decrease by one) the byte at the data pointer.
        """
        self.cell = (self.cell - 1) % 255

    def outp(self):
        """
        Output the byte at the data pointer.
        """
        self.stdout += chr(self.cell)

    def inp(self):
        """
        Accept one byte of input, storing its value in the byte at the data pointer.
        """
        char, *self.stdin = self.stdin
        self.cell = ord(char)

    def start_loop(self):
        """
        If the byte at the data pointer is zero,
        then instead of moving the instruction pointer forward to the next command,
        jump it forward to the command after the matching ] command.
        """
        if self.cell:
            self.stack.append(self.i)
        else:
            n = -1
            while self.code[self.i] != ']' or n:
                instruction = self.code[self.i]
                if instruction == '[':
                    n += 1
                elif instruction == ']':
                    n -= 1
                self.i += 1

    def end_loop(self):
        """
        If the byte at the data pointer is nonzero,
        then instead of moving the instruction pointer forward to the next command,
        jump it back to the command after the matching [ command.
        """
        if self.cell:
            self.i = self.stack.pop() - 1

    def execute(self):
        """
        Execute the function at the data pointer.
        """
        function = Interpreter(
            self.function_cell, self.pointer,
            self.stdin, self.stdout,
            self.stack, self.cells,
            self.function_cells, True
        )
        args = function()
        self.__dict__.update(args)

    def start_function(self):
        """
        Start writing a function.
        """
        self.function_cell = ''
        self.writing_function = True

    def end_function(self):
        """
        End writing a function
        """
        self.writing_function = False


if __name__ == '__main__':
    Interpreter(open('../test.fbf', mode='r+').read())()
