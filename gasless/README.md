# Gasless

## Usage in Node.js

1. Install by running:

```shell
cargo add gasless
```

2. Use the library to generate magic numbers

```rust
use gasless::*;

pub func sendTransaction() {
    // Setup Logic in Rust
    let gas_price = mine_free_gas(100_000, "0x...", 0);
    // Send Transaction in Alloy-rs, ethers-rs, etc
    // Set gas_price as gas price to use gasless tx
}
...
// Send a transaction in any library or manual JSON-RPC Request using the magic number as the gas price
```

> Notice, this library may not work with libraries since it's unique to SKALE and allows for transactions to be sent WITHOUT having any gas in the wallet


## Contributing

1. Fork and clone the project: https://github.com/Eidolon-Labs/gasless-rs
2. Create your changes
3. Make a Pull Request

## Security & Liability

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.