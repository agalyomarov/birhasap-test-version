import { ModalTypeEnum } from "~/enums/modal-type-enum";

export const useModal = () => {
  type OpenModalParams = {
    modalTitle: null | string;
    modalContent: string;
    modalType: ModalTypeEnum;
    onConfirm?: () => void;
    onCancel?: () => void;
  };

  const isOpen = useState("modal-open", () => false);
  const title = useState<string | null>("modal-title", () => "");
  const content = useState("modal-content", () => "");
  const type = useState<ModalTypeEnum>("modal-type", () => ModalTypeEnum.Warning);

  const confirmCallback = useState<(() => void) | null>("modal-confirm", () => null);
  const cancelCallback = useState<(() => void) | null>("modal-cancel", () => null);

  const openModal = ({ modalTitle, modalContent, modalType, onConfirm, onCancel }: OpenModalParams) => {
    title.value = modalTitle;
    content.value = modalContent;
    isOpen.value = true;
    type.value = modalType;

    confirmCallback.value = onConfirm ?? null;
    cancelCallback.value = onCancel ?? null;
  };

  const closeModal = () => {
    isOpen.value = false;
    title.value = "";
    content.value = "";

    confirmCallback.value = null;
    cancelCallback.value = null;
  };

  const confirm = () => {
    confirmCallback.value?.();
    closeModal();
  };

  const cancel = () => {
    cancelCallback.value?.();
    closeModal();
  };

  return {
    isOpen,
    title,
    content,
    type,
    openModal,
    closeModal,
    confirm,
    cancel,
  };
};
