// dental-service-dialog.models.ts
import {DentalService} from '../../../../api_services/dental-services-service';
import {DentalServiceType} from '../../../../api_services/dental-service-types-service';


export type DentalServiceDialogMode = 'create' | 'edit';

export interface DentalServiceDialogData {
  mode: DentalServiceDialogMode;
  service?: DentalService;                 // present in edit
  typeOptions: DentalServiceType[] ;  // dropdown source
  currentUserName?: string;                // optional, for defaults
}

export interface DentalServiceDialogResult {
  action: 'save' | 'cancel' | 'delete';
  // Payload to POST/PATCH (you can shape this to your API)
  payload?: Partial<DentalService>;
}
