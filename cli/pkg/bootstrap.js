import init, { main2 } from './sandstorm_cli.js';

async function run() {
    await init();
    /*****************
     * verify button *
     *****************/
    document.getElementById('verifyButton').addEventListener('click', async () => {
        let program_json = '';
        let public_input_json = '';
        let proof = '';
        document.getElementById('result').innerText = "Waiting for verification...";
        document.getElementById('result1').innerText = "";
        
        try {
            const programResponse = await fetch(document.getElementById('program').files[0].name);
            if (programResponse.ok) {
                program_json = await programResponse.text();
            }
        } catch (error) {
            console.error('Error fetching program JSON:', error);
        }

        try {
            const publicInputResponse = await fetch(document.getElementById('publicInput').files[0].name);
            if (publicInputResponse.ok) {
                public_input_json = await publicInputResponse.text();
            }
        } catch (error) {
            console.error('Error fetching public input JSON:', error);
        }

        try {
            const proofResponse = await fetch(document.getElementById('proof').files[0].name);
            if (proofResponse.ok) {
                const arrayBuffer = await proofResponse.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);
                proof = uint8Array;
            }
        } catch (error) {
            console.error('Error fetching proof file:', error);
        }

        const start = performance.now();
        try {
            let result = main2("Verify", program_json, public_input_json, proof, null, null, null);
            const end = performance.now();
            const elapsed = end - start;
            console.log("result: ", result);

            document.getElementById('result').innerText = "Verification successful! (elapsed time: " + elapsed + " ms)";
        } catch (error) {
            const end = performance.now();
            const elapsed = end - start;
            document.getElementById('result').innerText = "Verification failed!(elapsed time: " + elapsed + " ms)";
        }
    });

    /*****************
     * proof button  *
     *****************/
    document.getElementById('proofButton').addEventListener('click', async () => {
        let program_json = '';
        let public_input_json = '';
        let private_input_json = '';
        let trace = '';
        let memory = '';
        document.getElementById('result1').innerText = "Waiting for proof...";
        document.getElementById('result').innerText = "";

        let programFilename = document.getElementById('program1').files[0].name;

        try {
            const programResponse = await fetch(programFilename);
            if (programResponse.ok) {
                program_json = await programResponse.text();
            }
        } catch (error) {
            console.error('Error fetching program JSON:', error);
        }

        try {
            const publicInputResponse = await fetch(document.getElementById('publicInput1').files[0].name);
            if (publicInputResponse.ok) {
                public_input_json = await publicInputResponse.text();
            }
        } catch (error) {
            console.error('Error fetching public input JSON:', error);
        }

        try {
            const privateInputResponse = await fetch(document.getElementById('privateInput1').files[0].name);
            if (privateInputResponse.ok) {
                private_input_json = await privateInputResponse.text();
            }
        } catch (error) {
            console.error('Error fetching private input JSON:', error);
        }

        try {
            const traceResponse = await fetch(document.getElementById('trace1').files[0].name);
            if (traceResponse.ok) {
                const arrayBuffer = await traceResponse.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);
                trace = uint8Array;
            }
        } catch (error) {
            console.error('Error fetching trace file:', error);
        }

        try {
            const memoryResponse = await fetch(document.getElementById('memory1').files[0].name);
            if (memoryResponse.ok) {
                const arrayBuffer = await memoryResponse.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);
                memory = uint8Array;
            }
        } catch (error) {
            console.error('Error fetching memory file:', error);
        }

        const start = performance.now();
        try {
            let result = main2("Prove", program_json, public_input_json, null, trace, memory, private_input_json);
            const end = performance.now();
            const elapsed = end - start;
            console.log("result: ", result);
            let hexString = result; 
            let buffer = new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));

            let blob = new Blob([buffer], { type: 'application/octet-stream' });

            let link = document.createElement('a');
            link.href = window.URL.createObjectURL(blob);
            link.download = programFilename.split('.').slice(0, -1).join('.') + '.proof';
            link.click();

            document.getElementById('result1').innerText = "Proof generation successful! (elapsed time: " + elapsed + " ms)";
        } catch (error) {
            const end = performance.now();
            const elapsed = end - start;
            document.getElementById('result1').innerText = "Proof generation failed!(elapsed time: " + elapsed + " ms)";
        }
    });
}

run();