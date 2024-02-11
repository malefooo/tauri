<script lang="ts">
import {
  Document, FolderOpen, CloudUploadOutline, CubeSharp, BarChartOutline, CloudDownloadOutline,
  CodeDownloadSharp,ReloadCircleOutline,Options, AlertCircleOutline, GitPullRequest,RocketOutline,
  ReaderOutline, ResizeOutline,PlaySharp,
} from '@vicons/ionicons5'
import {computed, defineComponent, ref} from 'vue'
import { createDiscreteApi,   ConfigProviderProps, darkTheme} from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

const kmlOrExcelInput = ref('')
const kmlOutput = ref('')

const photoHandleInput = ref('')
const photoHandleRadius = ref('')
const photoHandleOutput = ref('')

const lineTreeData = ref([''])
const photoCalculateTreeData = ref([''])

// const themeRef = ref<'light' | 'dark'>('dark')
const configProviderPropsRef = computed<ConfigProviderProps>(() => ({
  theme: darkTheme
}))

const { message, dialog } = createDiscreteApi(
    ['message', 'dialog'],
    {
      configProviderProps: configProviderPropsRef.value
    }
)

async function calcPhoto() {
  console.log(photoHandleRadius.value)
  if (photoHandleRadius.value == '' || photoHandleRadius.value == '0') {
    dialog.error({title: "请输入半径"})
    return
  }

  if (photoHandleInput.value == '') {
    dialog.error({title: "请输入导入照片文件夹路径"})
    return
  }

  try {
    const data = await invoke("calc_photo", {radius: photoHandleRadius.value, photoPath: photoHandleInput.value});
    console.log(data);
    const treeData = JSON.parse(typeof data === "string" ? data :"");
    photoCalculateTreeData.value = treeData;
    message.success("success")
  } catch(error) {
    dialog.error({title: typeof error === "string" ? error :""})
  }
}

async function moveToOutput() {
  if (photoHandleOutput.value == '') {
    dialog.error({title: "请输入导出照片文件夹路径"})
    return
  }

  try {
    const v = await invoke("move_to_output", {output: photoHandleOutput.value});
    console.log(v);
    message.success("success")
  } catch(error) {
    dialog.error({title:  typeof error === "string" ? error :""})
  }
}

async function exportExcel() {
  const isKmlFile = kmlOrExcelInput.value.endsWith('.kml');
  if (!isKmlFile) {
    dialog.error({title: "请选择kml文件"})
    return
  }

  if (kmlOutput.value == "") {
    dialog.error({title: "请选择excel输出文件夹"})
    return
  }

  try {
    const v = await invoke("kml_to_excel", {kmlFile: kmlOrExcelInput.value, outputDir: kmlOutput.value});
    console.log(v);
    message.success("success")
  } catch(error) {
    dialog.error({title:  typeof error === "string" ? error :""})
  }

}

async function kmlToTree() {
  try {
    const isKmlFile = kmlOrExcelInput.value.endsWith('.kml');
    const data = ref("")
    if (isKmlFile) {
      data.value = await invoke("kml_to_json", {kmlFile: kmlOrExcelInput.value});
    } else {
      data.value = await invoke("excel_to_json", {excelFile: kmlOrExcelInput.value});
    }

    console.log(data);
    const treeData = JSON.parse(data.value);
    lineTreeData.value = treeData;
    message.success("success")
  } catch(error) {
    dialog.error({title:  typeof error === "string" ? error :""})
  }

}

async function openFileDialogForKmlOrExcelInput() {
  const selected = await open({
    // 配置选项
    filters: [
      {
        name: 'KML 或 Excel Files',
        extensions: ['kml','xlsx']
      }
    ]
  });
  console.log(selected);
  if (selected) {
    typeof selected === "string" ? kmlOrExcelInput.value = selected :"";
  }
}

async function openFolderDialogForPhotoInput() {
  try {
    const selected = await open({
      directory: true, // 设置为true来选择文件夹
      multiple: false  // 可以设置为true如果您想允许选择多个文件夹
    });
    if (selected) {
      console.log('选择的文件夹:', selected);
      typeof selected === "string" ? photoHandleInput.value = selected : ""
    }
  } catch (error) {
    console.error('选择文件夹出错', error);
  }
}

async function openFolderDialogForKmlOutput() {
  try {
    const selected = await open({
      directory: true, // 设置为true来选择文件夹
      multiple: false  // 可以设置为true如果您想允许选择多个文件夹
    });
    if (selected) {
      console.log('选择的文件夹:', selected);
      typeof selected === "string" ? kmlOutput.value = selected : ""
    }
  } catch (error) {
    console.error('选择文件夹出错', error);
  }
}

async function openFolderDialogForPhotoOutput() {
  try {
    const selected = await open({
      directory: true, // 设置为true来选择文件夹
      multiple: false  // 可以设置为true如果您想允许选择多个文件夹
    });
    if (selected) {
      console.log('选择的文件夹:', selected);
      typeof selected === "string" ? photoHandleOutput.value = selected : ""
    }
  } catch (error) {
    console.error('选择文件夹出错', error);
  }
}

export default defineComponent({
  components: {
  },
  setup () {
    return {
      Document, FolderOpen, CloudUploadOutline, CubeSharp, BarChartOutline, CloudDownloadOutline, CodeDownloadSharp,
      ReloadCircleOutline,Options, AlertCircleOutline, GitPullRequest,ReaderOutline, ResizeOutline, RocketOutline,PlaySharp,
      lineTreeData: lineTreeData,
      photoCalculateTreeData: photoCalculateTreeData,
      onlyAllowNumber: (value: string) => !value || /^\d+$/.test(value),
      exportExcel: exportExcel,
      photoHandleRadiusChange: (value: string) => {
        photoHandleRadius.value = value;
      },
      photoHandleInput: photoHandleInput,
      photoHandleOutput: photoHandleOutput,
      openFileDialogForKmlOrExcelInput: openFileDialogForKmlOrExcelInput,
      openFolderDialogForKmlOutput: openFolderDialogForKmlOutput,
      kmlOrExcelInput: kmlOrExcelInput,
      kmlOutput: kmlOutput,
      kmlToTree: kmlToTree,
      moveToOutput: moveToOutput,
      calcPhoto: calcPhoto,
      openFolderDialogForPhotoInput:openFolderDialogForPhotoInput,
      openFolderDialogForPhotoOutput:openFolderDialogForPhotoOutput,
    }
  }
})

</script>


<template>

    <n-space vertical size="large">

      <n-layout class="bgc" >
        <n-layout-header class="head-font-color n-layout-header-standard">
          <n-icon :component="CubeSharp"></n-icon>  KML文件处理
        </n-layout-header>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudUploadOutline"></n-icon>
                </template>
                导入
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="kmlOrExcelInput"
                       @click="openFileDialogForKmlOrExcelInput"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f;"
                       round
                       placeholder="文件: ../XXX/XXX.KML 或 XXX.xlsx" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudDownloadOutline"></n-icon>
                </template>
                输出
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="kmlOutput"
                       @click="openFolderDialogForKmlOutput"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f"
                       round placeholder="文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="Options"></n-icon>
                </template>
                操作
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button @click="exportExcel"  color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="CodeDownloadSharp"></n-icon>
                </template>
                导出excel
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button @click="kmlToTree" color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="ReloadCircleOutline"></n-icon>
                </template>
                导入线路数据
              </n-button>
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-footer></n-layout-footer>
      </n-layout>

      <n-layout class="bgc">
        <n-layout-header class="head-font-color n-layout-header-standard">
          <n-icon :component="CubeSharp"></n-icon>  图片处理
        </n-layout-header>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi :span="7">
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="AlertCircleOutline"></n-icon>
                </template>
                说明:在点击计算后,会根据站点经纬度和半径画圆,然后判断照片的经纬度时候属于圆内
              </n-button>
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">

            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudUploadOutline"></n-icon>
                </template>
                导入
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="photoHandleInput"
                       @click="openFolderDialogForPhotoInput"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f"
                       round
                       placeholder="文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">

            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="ResizeOutline"></n-icon>
                </template>
                半径
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :allow-input="onlyAllowNumber"
                       @change="photoHandleRadiusChange"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f" round placeholder="站点辐射的范围半径, 单位: 米" />
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudDownloadOutline"></n-icon>
                </template>
                输出
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="photoHandleOutput"
                       @click="openFolderDialogForPhotoOutput"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f"
                       round
                       placeholder="文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="Options"></n-icon>
                </template>
                操作
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button @click="calcPhoto" color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="RocketOutline"></n-icon>
                </template>
                计算
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button @click="moveToOutput" color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="PlaySharp"></n-icon>
                </template>
                开始
              </n-button>
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-footer></n-layout-footer>
      </n-layout>

      <n-layout has-sider class="bgc">

        <n-layout class="bgc">
          <n-layout-header class="head-font-color n-layout-header-standard">
            <n-icon :component="BarChartOutline"></n-icon>  数据展示
          </n-layout-header>
          <n-layout-content content-style="padding: 10px;"  class="content-font-color">
            <n-grid x-gap="5" :cols="2" >
              <n-gi>
                <n-layout class="bgc">
                  <n-layout-header class="n-layout-header-internal">
                    <n-grid x-gap="10" :cols="3" >
                      <n-gi :span="3">
                        <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                          <template #icon>
                            <n-icon :component="CloudDownloadOutline"></n-icon>
                          </template>
                          导入的线路数据
                        </n-button>
                      </n-gi>
                    </n-grid>
                  </n-layout-header>
                  <n-layout-content style="height: 300px" content-style="padding: 10px;" class="content-font-color">
                    <n-tree
                        block-line
                        default-expand-all
                        :data="lineTreeData"
                        selectable
                        style="font-weight: 900; --n-node-text-color: burlywood"
                    />
                  </n-layout-content>
                  <n-layout-footer></n-layout-footer>
                </n-layout>
              </n-gi>
              <n-gi>
                <n-layout class="bgc">
                  <n-layout-header class="n-layout-header-internal">
                    <n-grid  x-gap="5" :cols="3">
                      <n-gi :span="3">
                        <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                          <template #icon>
                            <n-icon :component="ReaderOutline"></n-icon>
                          </template>
                          导入的照片数计算结果
                        </n-button>
                      </n-gi>
                    </n-grid>
                  </n-layout-header>
                  <n-layout-content style="height: 300px" content-style="padding: 10px;"  class="content-font-color">
                    <n-tree
                        block-line
                        default-expand-all
                        :data="photoCalculateTreeData"
                        selectable
                        style="font-weight: 900; --n-node-text-color: burlywood"
                    />
                  </n-layout-content>
                  <n-layout-footer></n-layout-footer>
                </n-layout>
              </n-gi>
            </n-grid>

          </n-layout-content>
          <n-layout-footer></n-layout-footer>
        </n-layout>
      </n-layout>

    </n-space>

</template>

<style scoped>

.center-text {
  text-align: center;
}

.center-text-vertical {
  display: flex;
  justify-content: center; /* 水平居中 */
  align-items: center;     /* 垂直居中 */
  height: 100px;           /* 或者根据需要设置高度 */
}

.bgc {
  background-color: #2f2f2f
}

.n-layout-header-standard {
  background: rgba(128, 128, 128, 0.2);
  padding: 2px;
}

.n-layout-header-internal {
  background: rgba(128, 128, 128, 0.4);
  padding: 2px;
}

.n-layout-footer {
  background: rgba(128, 128, 128, 0.2);
  padding: 2px;
}

.n-layout-sider {
  background: rgba(128, 128, 128, 0.3);
}

.n-layout-content {
  background: rgba(128, 128, 128, 0.4);
}

.head-font-color {
  color: wheat;
}

.content-font-color {
  color: burlywood;
}

.wheat1 {
  height: 30px;
  background-color: wheat;
}
.burlywood1 {
  height: 30px;
  background-color: burlywood;
}
.ngi-font-color {
  color: #2f2f2f;
}
.ngi-font-color-burlywood {
  color: burlywood;
}
</style>