class Interpreter:
    def __init__(self,
                 code: str,

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

        self.pointer = 0

        self.in_comment: bool = False
        self.writing_function: bool = False

    @property
    def cell(self):
        return self.cells[self.pointer]

    @cell.setter
    def cell(self, value):
        self.cells[self.pointer] = value

    @property
    def function_cell(self):
        return self.function_cells[self.pointer]

    def __call__(self, *args, **kwargs) -> tuple[str, str]:
        while self.i < len(self.code):
            instruction = self.code[self.i]

            if instruction == '#':
                self.i += 1
                self.in_comment = not self.in_comment
                continue
            if self.in_comment:
                self.i += 1
                continue

            if instruction == ':':
                self.function()
            elif self.writing_function:
                self.function_cells[self.pointer] += instruction
                self.i += 1
                continue

            if instruction == '>':
                self.right()
            elif instruction == '<':
                self.left()

            elif instruction == '+':
                self.add()
            elif instruction == '-':
                self.sub()

            elif instruction == '.':
                self.outp()
            elif instruction == ',':
                self.inp()

            elif instruction == '[':
                self.start_loop()
            elif instruction == ']':
                self.end_loop()

            elif instruction == 'x':
                self.execute()
            self.i += 1

        if self.is_function:
            return self.stdin, self.stdout
        else:
            print(self.stdout)

    def function(self):
        """
        Write the code between the matching : and : to the function cell at the data pointer.
        """
        self.writing_function = not self.writing_function

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
        self.i = self.stack.pop() - 1

    def execute(self):
        """
        Execute the function at the data pointer.
        """
        function = Interpreter(
            self.function_cell,
            self.stdin, self.stdout,
            self.stack, self.cells,
            self.function_cells, is_function=True
        )
        self.stdin, self.stdout = function()
