class Interpreter:
    def __init__(self,
                 code: str,

                 stdin: str = None,
                 stdout: str = '',

                 stack: list = None,
                 cells: list[int] = None,
                 ):
        self.i = 0
        self.code = code.replace('\n', '').replace('\t', '').replace(' ', '')
        if ',' in self.code and not stdin:
            stdin = [*input()]

        self.stack = stack or []
        self.cells = cells or [0] * 30000

        self.stdin = stdin
        self.stdout = stdout

        self.pointer = 0

        self.in_comment: bool = False

    @property
    def cell(self):
        return self.cells[self.pointer]

    @cell.setter
    def cell(self, value):
        self.cells[self.pointer] = value

    def __call__(self, *args, **kwargs):
        while self.i < len(self.code):
            instruction = self.code[self.i]

            if instruction == '#':
                self.i += 1
                self.in_comment = not self.in_comment
                continue
            if self.in_comment:
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
            self.i += 1

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
        self.i = self.stack.pop() - 1
