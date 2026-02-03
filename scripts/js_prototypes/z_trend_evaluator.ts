/**
 * Copyright 2026 Pecos D. Willy
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *     http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

export type ClientData = {
    id: string;
    name: string;
    age: number;
    previousAge?: number;
    lastUpdated: string;
    dataPoints: {
        R: number; O: number; Y: number; G: number; B: number; I: number; V: number;
        R_prime: number; O_prime: number; Y_prime: number; G_prime: number; B_prime: number; I_prime: number; V_prime: number;
    };
    metadata: string;
    schedule: string;
};

export type AuditConfig = {
    zScoreThreshold: number;
    enableMultivariate: boolean;
    multivariateThreshold: number;
    activeRules: string[];
};

export const detectAnomalies = (clients: ClientData[], config: AuditConfig) => {
    const anomalies = new Map<string, string[]>(); // clientId -> list of anomaly descriptions

    const cols = ['R', 'O', 'Y', 'G', 'B', 'I', 'V'];
    const valuesMatrix = clients.map(c => cols.map(col => c.dataPoints[col as keyof typeof c.dataPoints]));

    // 1. Column-wise Z-Score Analysis (Univariate)
    cols.forEach((col, colIndex) => {
        const values = valuesMatrix.map(row => row[colIndex]);

        // Mean
        const sum = values.reduce((a, b) => a + b, 0);
        const mean = sum / values.length;

        // StdDev
        const variance = values.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / values.length;
        const stdDev = Math.sqrt(variance);

        clients.forEach((client, clientIndex) => {
            const val = values[clientIndex];
            const zScore = stdDev === 0 ? 0 : (val - mean) / stdDev;

            if (Math.abs(zScore) > config.zScoreThreshold) {
                const current = anomalies.get(client.id) || [];
                current.push(`Univariate Outlier in ${col}: Value ${val} is ${zScore.toFixed(2)}σ from mean.`);
                anomalies.set(client.id, current);
            }
        });
    });

    // 2. Multivariate Analysis (Euclidean Distance from Centroid)
    if (config.enableMultivariate) {
        // Calculate Centroid (Mean Vector)
        const centroid = cols.map((_, colIdx) => {
            const colValues = valuesMatrix.map(row => row[colIdx]);
            return colValues.reduce((a, b) => a + b, 0) / colValues.length;
        });

        // Calculate distances
        const distances = valuesMatrix.map(row => {
            const sumSqDiff = row.reduce((acc, val, idx) => acc + Math.pow(val - centroid[idx], 2), 0);
            return Math.sqrt(sumSqDiff);
        });

        // Calculate Mean Distance and StdDev of Distances
        const meanDist = distances.reduce((a, b) => a + b, 0) / distances.length;
        const varDist = distances.reduce((a, b) => a + Math.pow(b - meanDist, 2), 0) / distances.length;
        const stdDist = Math.sqrt(varDist);

        clients.forEach((client, idx) => {
            const dist = distances[idx];
            const zScoreDist = stdDist === 0 ? 0 : (dist - meanDist) / stdDist;

            if (zScoreDist > config.multivariateThreshold) {
                const current = anomalies.get(client.id) || [];
                current.push(`Multivariate Anomaly: Client data vector deviates significantly (${zScoreDist.toFixed(2)}σ) from population centroid.`);
                anomalies.set(client.id, current);
            }
        });
    }

    return anomalies;
};
