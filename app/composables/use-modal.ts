export const useModal = () => {
  type ModalType = "warning" | "danger";
  const isOpan = useState("modal-open", () => false);
  const title = useState<string | null>("modal-title", () => "");
  const content = useState("modal-content", () => "");
  const type = useState<ModalType>("modal-type", () => "warning");

  const openModal = ({ modalTitle, modalContent, modalType }: { modalTitle: null | string; modalContent: string; modalType: ModalType }) => {
    title.value = modalTitle;
    content.value = modalContent;
    isOpan.value = true;
    type.value = modalType;
  };

  const closeModal = () => {
    isOpan.value = false;
    title.value = "";
    content.value = "";
  };

  return {
    isOpan,
    title,
    content,
    type,
    openModal,
    closeModal,
  };
};
