const std = @import("std");

const Direction = enum {
    left,
    right,
};

const Command = struct {
    value: u32,
    direction: Direction,

    fn parse(buf: []const u8) !Command {
        const direction = switch (buf[0]) {
            'L' => Direction.left,
            'R' => Direction.right,
            else => @panic("Unknown direction"),
        };
        const value = try std.fmt.parseInt(u32, buf[1..], 10);

        return Command{
            .direction = direction,
            .value = value,
        };
    }
};

const Lock = struct {
    value: u32 = 50,

    fn left(self: *Lock) void {
        if (self.value == 0) {
            self.value = 99;
        } else {
            self.value -= 1;
        }
    }

    fn right(self: *Lock) void {
        self.value = (self.value + 1) % 100;
    }
};

pub fn run(part2: bool) !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    defer _ = gpa.detectLeaks();

    const alloc = gpa.allocator();

    var lock = Lock{};

    const file_data = try std.fs.cwd().readFileAlloc(alloc, "./src/day1/input.txt", std.math.maxInt(usize));
    defer alloc.free(file_data);

    var count: u32 = 0;

    var it = std.mem.splitScalar(u8, file_data, '\n');
    while (it.next()) |x| {
        const trimmed = std.mem.trim(u8, x, " \n");
        if (trimmed.len == 0) continue;

        const command = try Command.parse(trimmed);

        if (part2) {
            for (0..command.value) |_| {
                switch (command.direction) {
                    Direction.left => lock.left(),
                    Direction.right => lock.right(),
                }

                if (lock.value == 0) {
                    count += 1;
                }
            }
        } else {
            for (0..command.value) |_| {
                switch (command.direction) {
                    Direction.left => lock.left(),
                    Direction.right => lock.right(),
                }
            }

            if (lock.value == 0) {
                count += 1;
            }
        }
    }

    std.debug.print("count: {d}\n", .{count});
}
