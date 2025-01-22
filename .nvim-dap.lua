local dap = require("dap")

dap.configurations.rust = {
    {
        name = "debug service",
        type = "lldb",
        request = "launch",
        options = {
            env = {
                RUST_LOG = "debug",
                PGSSLMODE = "disable",
                PGHOST = "127.0.0.1",
                PGUSER = "postgres",
                PGPORT = "55432",
                PATH_FIELD = "$.url",
                BASE_URL = "http=//localhost=8080",
            }
        },
        program = function()
            vim.fn.jobstart('ls')
            return vim.fn.getcwd() .. "/target/debug/pm-huawei-rs"
        end,
        cwd = "${workspaceFolder}",
        args = {"service",},
        stopOnEntry = false,
    },
    {
        name = "hello-dap",
        type = "lldb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/pm-huawei-rs"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
    },
}
