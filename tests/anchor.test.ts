// No imports needed: web3, anchor, pg and more are globally available

describe("Tienda de Vinilos", () => {
  it("crear tienda", async () => {
    // Nombre de la tienda
    const nombreTienda = "Tienda de Vinilos";

    // Derivar la PDA de la tienda
    const [tiendaPda] = await web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("tienda_vinilos"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    // Enviar transacción
    const txHash = await pg.program.methods
      .crearTienda(nombreTienda)
      .accounts({
        owner: pg.wallet.publicKey,
        tienda: tiendaPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirmar transacción
    await pg.connection.confirmTransaction(txHash);

    // Obtener la cuenta creada
    const tienda = await pg.program.account.tienda.fetch(tiendaPda);

    console.log("Datos on-chain:", tienda);

    // Verificar que el nombre se guardó correctamente
    assert.equal(tienda.nombre, nombreTienda);
  });
});

