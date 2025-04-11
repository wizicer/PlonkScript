<template>
  <q-page class="">
    <q-splitter
      v-model="splitPercent"
      style="height: 90vh"
      @update:model-value="updatePanel()"
      :limits="[0, Infinity]"
    >
      <template v-slot:before>
        <div class="q-pa-md">
          <!-- File tabs -->
          <div class="row q-mb-md justify-between">
            <div v-if="files.length > 1" style="flex-grow: 1">
              <q-tabs
                v-model="activeTab"
                dense
                class="text-grey"
                active-color="primary"
                indicator-color="primary"
                align="left"
                narrow-indicator
              >
                <q-tab
                  v-for="file in files"
                  :key="file.id"
                  :name="file.id"
                  :label="file.name"
                />
              </q-tabs>
            </div>
            <div v-else></div>
            <div>
              <q-btn flat round dense icon="add" @click="addNewFile" />
            </div>
          </div>

          <!-- File name input -->
          <div v-if="activeFile && files.length > 1" class="row q-mb-sm">
            <q-input
              v-model="activeFile.name"
              dense
              class="q-mr-sm"
              style="width: 200px"
              label="File name"
            />
            <q-btn
              flat
              round
              dense
              icon="delete"
              color="negative"
              @click="removeCurrentFile"
              :disable="activeFile.id === 'main'"
            />
          </div>

          <!-- Single editor container -->
          <div
            ref="editorContainer"
            id="editorContainer"
            style="height: 75vh; border: 1px solid #ddd"
          ></div>
        </div>
      </template>

      <template v-slot:after>
        <div class="q-pa-md">
          <ConstraintsVisualization :data="vis" />
        </div>
      </template>
    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref, watch, computed } from 'vue';
import * as monaco from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import { language, theme } from 'src/services/PlonkScriptLanguage';
import init, { try_run } from '../transpiler';
import { convertMockProverOutputToObject } from 'src/services/MockProverTranslator';
import { MockProverData } from 'src/services/ConstraintSystem';
import ConstraintsVisualization from '../components/ConstraintsVisualization.vue';

self.MonacoEnvironment = {
  getWorker() {
    return new editorWorker();
  },
};

monaco.languages.register({ id: 'plonkscript' });
monaco.languages.setMonarchTokensProvider('plonkscript', language);
monaco.editor.defineTheme('plonkscript', theme);

const mainCode = `# k: 4
# in1: 1
# in2: 1

region first_row(a, b, c, in1, in2) {
    a[0] <== in1;
    b[0] <== in2;
    c[0] <== a[0] + b[0];

    [b[0], c[0]]
}

region next_row(a, b, c, last_b, last_c) {
    a[0] <== last_b;
    b[0] <== last_c;
    c[0] <== a[0] + b[0];

    c[0]
}

let N = 10;

pub input in1;
pub input in2;
pub output out;

col advice a;
col advice b;
col advice c;

let fr = first_row(a, b, c, in1, in2);
let last_b = fr[0];
let last_c = fr[1];
for i in 1..N {
    let c = next_row(a, b, c, last_b, last_c);
    last_b = last_c;
    last_c = c;
}

out <== last_c;

// Press Ctrl+Enter to execute the code
`;

const splitPercent = ref(50);
const activeTab = ref('main');
const editorContainer = ref<HTMLElement | null>(null);

interface FileData {
  id: string;
  name: string;
  content: string;
  model?: monaco.Uri;
}

const files = ref<FileData[]>([
  { id: 'main', name: 'main.plonk', content: mainCode },
]);

const activeFile = computed(() => {
  return files.value.find((f) => f.id === activeTab.value);
});

let editor: monaco.editor.IStandaloneCodeEditor | null = null;
const vis: Ref<MockProverData | undefined> = ref(undefined);

let nextId = 1;
function generateId() {
  return `file-${nextId++}`;
}

function initEditor() {
  const container = editorContainer.value;
  if (!container) {
    console.error('Editor container not found');
    return;
  }

  editor = monaco.editor.create(container, {
    value: '',
    language: 'plonkscript',
    theme: 'plonkscript',
    minimap: { enabled: false },
  });

  editor.addAction({
    id: 'runCode',
    label: 'RunCode',
    keybindings: [
      monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyR,
      monaco.KeyMod.WinCtrl | monaco.KeyCode.Enter,
      monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter,
    ],
    contextMenuGroupId: 'debug',
    contextMenuOrder: 1.5,
    run: runCode,
  });

  for (const file of files.value) {
    file.model = monaco.editor.createModel(file.content, 'plonkscript').uri;
  }

  if (activeFile.value && activeFile.value.model) {
    editor.setModel(monaco.editor.getModel(activeFile.value.model));
  }
}

function updateEditorModel() {
  if (!editor || !activeFile.value) return;

  try {
    if (!activeFile.value.model) {
      activeFile.value.model = monaco.editor.createModel(
        activeFile.value.content,
        'plonkscript'
      ).uri;
    }

    editor.setModel(monaco.editor.getModel(activeFile.value.model));
    editor.focus();
  } catch (error) {
    console.error('Error updating editor model:', error);
  }
}

function runCode() {
  try {
    const mainFile = files.value.find((f) => f.id === 'main');
    if (!mainFile) {
      console.error('Main file not found');
      return;
    }

    for (const file of files.value) {
      if (file.model) {
        const model = monaco.editor.getModel(file.model);
        if (model) {
          file.content = model.getValue();
        }
      }
    }

    const modules: Record<string, string> = {};
    for (const file of files.value) {
      if (file.id !== 'main') {
        const moduleName = file.name.replace(/\.plonk$/, '');
        modules[moduleName] = file.content;
      }
    }

    const result = try_run({
      code: mainFile.content,
      modules: modules,
    });

    vis.value = convertMockProverOutputToObject(result);
  } catch (error) {
    console.error('Error running code:', error);
  }
}

function addNewFile() {
  const fileId = generateId();
  const newContent = '// New module file';

  const newFile: FileData = {
    id: fileId,
    name: `module${files.value.length}.plonk`,
    content: newContent,
  };

  if (monaco.editor) {
    newFile.model = monaco.editor.createModel(newContent, 'plonkscript').uri;
  }

  files.value.push(newFile);
  activeTab.value = fileId;
}

function removeCurrentFile() {
  if (
    !activeFile.value ||
    activeFile.value.id === 'main' ||
    files.value.length <= 1
  ) {
    return;
  }

  const fileIdToRemove = activeFile.value.id;
  const index = files.value.findIndex((f) => f.id === fileIdToRemove);
  if (index === -1) return;

  if (activeFile.value.model) {
    const model = monaco.editor.getModel(activeFile.value.model);
    if (model) {
      model.dispose();
    }
  }

  files.value.splice(index, 1);
  activeTab.value = 'main';
}

function updatePanel() {
  if (!editor) return;
  editor.layout();
}

init();

setTimeout(() => {
  initEditor();
}, 300);

watch(activeTab, () => {
  updateEditorModel();
});
</script>
