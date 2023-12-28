import init, { main2 } from './sandstorm_cli.js';

async function run() {
    await init();

    document.getElementById('calculateButton').addEventListener('click', async () => {
        let program_json = '';
        let public_input_json = '';
        let proof = '';

        try {
            const programResponse = await fetch('./array-sum.json');
            if (programResponse.ok) {
                program_json = await programResponse.text();
            }
        } catch (error) {
            console.error('Error fetching program JSON:', error);
        }

        try {
            const publicInputResponse = await fetch('./air-public-input.json');
            if (publicInputResponse.ok) {
                public_input_json = await publicInputResponse.text();
            }
        } catch (error) {
            console.error('Error fetching public input JSON:', error);
        }

        try {
            const proofResponse = await fetch('./array-sum.proof');
            if (proofResponse.ok) {
                const arrayBuffer = await proofResponse.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);
                proof = uint8Array;
            }
        } catch (error) {
            console.error('Error fetching proof file:', error);
        }

        //console.log('Program JSON:', program_json);
        //console.log('Public input JSON:', public_input_json);
        try {
            // get now previous to main2 and elapsed time after main2 call
            const start = performance.now();
            main2(program_json, public_input_json, proof);
            const end = performance.now();
            const elapsed = end - start;

            document.getElementById('result').innerText = "Verification successful! (elapsed time: " + elapsed + " ms)";
        } catch (error) {
            const end = performance.now();
            const elapsed = end - start;
            document.getElementById('result').innerText = "Verification failed!(elapsed time: " + elapsed + " ms)";
        }
        
        
    });
}

run();