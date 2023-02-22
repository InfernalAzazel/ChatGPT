<template>
  <el-form
      ref="formRef"
      :model="form"
      class="w-full pt-10"
  >
    <el-form-item prop="bing_ua" label="bing_ua">
      <el-input v-model="form.bing_ua"/>
    </el-form-item>
    <el-form-item prop="gpt_ua" label="gpt_ua">
      <el-input v-model="form.gpt_ua"/>
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="submitForm(formRef)">保存</el-button>
      <el-button @click="resetForm(formRef)">重置</el-button>
    </el-form-item>
  </el-form>

</template>
<script setup lang="ts">
import {reactive, ref} from "vue";
import type {FormInstance} from "element-plus";

const formRef = ref<FormInstance>();
const form = reactive({
  bing_ua:
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.0.0",
  gpt_ua:
      "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.1 (KHTML, like Gecko) Chrome/22.0.1207.1 Safari/537.1",
});

const submitForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  formEl.validate((valid) => {
    if (valid) {
      console.log("submit!");
    } else {
      console.log("error submit!");
      return false;
    }
  });
};

const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  formEl.resetFields();
};
</script>
<style scoped></style>