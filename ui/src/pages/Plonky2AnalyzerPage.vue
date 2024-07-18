<template>
  <q-page class="">
    <div class="q-pa-md">
      <div>
        <q-btn-toggle
          v-model="modelSelection"
          push
          toggle-color="primary"
          :options="([{ label: 'Custom', value: undefined }] as any).concat(dataList.map((_) => ({ label: _.name, value: _ })))"
        />
      </div>
      <q-card v-if="modelSelection && modelSelection.name != 'Custom'" class="">
        <q-card-section>
          <div v-if="modelSelection?.title" class="text-h6">
            {{ modelSelection.title }}
          </div>
        </q-card-section>

        <q-card-section v-if="modelSelection?.description"
          >{{ modelSelection.description }}
        </q-card-section>

        <q-separator v-if="modelSelection?.sourceUrl" />

        <q-card-actions>
          <q-btn
            flat
            v-if="modelSelection?.sourceUrl"
            :href="modelSelection.sourceUrl"
            target="_blank"
            :icon="matOpenInNew"
            >See Source Code</q-btn
          >
        </q-card-actions>
      </q-card>

      <q-card v-else class="">
        <q-card-section>
          <div class="text-h6">Plonky2 Analyzer for custom data</div>
        </q-card-section>

        <q-card-section>
          inject your code like this to get the trace for analysis:
          <pre>
let data = builder.build::&lt;C&gt;();
let proof = data.prove(pw.clone())?;

let partition_witness =
    plonky2::iop::generator::generate_partial_witness(pw, &data.prover_only, &data.common);

let _witness = format!("{:#?}", partition_witness);
let _data = format!("{:#?}", data);
let _proof = format!("{:#?}", proof);
let _output = format!("Plonky2Data {&#123;\nwitness: {},\ndata: {},\nproof: {}\n&#125;}", _witness, _data, _proof);
let mut file = std::fs::File::create("output.rust").unwrap();
std::io::Write::write_all(&mut file, _output.as_bytes()).unwrap();</pre
          >
          <!-- or
          <pre>
plonky2_summarizer = "0.1.0"
plonky2_summarizer ={path="/Users/oker/2-Project/00-zkptech/plonkscript/plonky2_summarizer/"}

let d = format!("{:#?}", prover);
let d = halo2_summarizer::trim(&d, Some(0..1024));
let mut file = std::fs::File::create("data.rust").unwrap();
std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();</pre
          > -->

          PS: this is client-only processing, no data is transfered to the
          server.
        </q-card-section>
        <q-separator />
        <q-card-actions>
          <q-uploader ref="uploaderRef" :multiple="false" @added="onFileAdded">
            <template v-slot:header="scope">
              <div class="row no-wrap items-center q-pa-sm q-gutter-xs">
                <div class="col">
                  <div class="q-uploader__title">Select the debug output</div>
                </div>
                <q-btn
                  v-if="scope.canAddFiles"
                  type="a"
                  icon="add_box"
                  @click="scope.pickFiles"
                  round
                  dense
                  flat
                >
                  <q-uploader-add-trigger />
                  <q-tooltip>Pick Files</q-tooltip>
                </q-btn>
              </div>
            </template>

            <template v-slot:list="">
              <q-list separator>
                <q-item v-if="convertedJson">
                  <q-item-section>
                    <q-item-label class="full-width ellipsis">
                      <a @click="save" href="javascript:void(0)"> data.json </a>
                    </q-item-label>

                    <q-item-label caption
                      >Click to save converted JSON</q-item-label
                    >
                  </q-item-section>
                </q-item>
              </q-list>
            </template>
          </q-uploader>
        </q-card-actions>
      </q-card>
    </div>

    <Plonky2Visualization :data="modelSelection?.data" />
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue';
import { matOpenInNew } from '@quasar/extras/material-icons';
import {
  IDataModel,
  Plonky2Data,
  dataList,
} from 'src/services/plonky2/DefaultModels';
import Plonky2Visualization from '../components/Plonky2Visualization.vue';
import { QUploader } from 'quasar';
import { useQuasar } from 'quasar';
import { convertMockProverOutputToJson } from 'src/services/MockProverTranslator';

const $q = useQuasar();

const modelSelection: Ref<IDataModel | undefined> = ref(undefined);
const uploaderRef: Ref<QUploader | null> = ref(null);

const convertedJson = ref('');

function onFileAdded(files: readonly File[]) {
  var reader = new FileReader();
  reader.onload = function (event) {
    uploaderRef.value?.reset();
    const result = event.target?.result;
    if (
      !result ||
      typeof result != 'string' ||
      !result.startsWith('Plonky2Data')
    ) {
      $q.notify({
        message: 'Invalid file, only MockProver debug output is supported.',
        type: 'negative',
      });
      return;
    }

    const json = convertMockProverOutputToJson(result);
    convertedJson.value = json;
    const data = JSON.parse(json) as Plonky2Data;
    modelSelection.value = { name: 'Custom', data };
  };
  reader.readAsText(files[0]);
}

function save() {
  const blob = new Blob([convertedJson.value], {
    type: 'application/json',
  });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'data.json';
  a.click();
  URL.revokeObjectURL(url);
}
</script>
