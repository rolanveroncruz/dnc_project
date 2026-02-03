// src/app/api_services/clinic-capabilities-list.service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

// Mirrors backend: clinic_capability::Model (add fields as needed)
export interface ClinicCapability {
  id: number;
  name?: string;
  description?: string;
  active?: boolean;
  last_modified_by?: string;
  last_modified_on?: string; // or Date, depending on your API format
  [key: string]: unknown;
}

// Mirrors backend: ClinicCapabilityLinkRow. Here is an example:
// {
//   "id": 3,
//   "clinic_id": 1,
//   "capability_id": 1,
//   "capability": {
//   "id": 1,
//     "name": "Dental Radiography (Panoramic)",
//     "active": true,
//     "last_modified_by": "system",
//     "last_modified_on": "2026-01-29T18:34:57.345940Z"
// }
export interface ClinicCapabilityLinkRow {
  id: number;
  clinic_id: number;
  capability_id: number;
  capability: ClinicCapability | null;
}

export interface AddClinicCapabilityBody {
  capability_id: number;
}

export interface SetClinicCapabilitiesBody {
  capability_ids: number[];
}

@Injectable({ providedIn: 'root' })
export class ClinicCapabilitiesListService {
  private readonly http = inject(HttpClient);
  private readonly loginService = inject(LoginService);
    private authHeaders() {
        let token = this.loginService.token();
        return  {'Authorization': `Bearer ${token}`};
    }

  // e.g. https://api.example.com
  private readonly baseUrl = `${environment.apiUrl}/api`;

  private clinicCapsUrl(clinicId: number): string {
    // GET/POST/PUT  /dental_clinics/:clinic_id/capabilities
    return `${this.baseUrl}/dental_clinics/${clinicId}/capabilities`;
  }

  private clinicCapUrl(clinicId: number, capabilityId: number): string {
    // DELETE /dental_clinics/:clinic_id/capabilities/:capability_id
    return `${this.baseUrl}/dental_clinics/${clinicId}/capabilities/${capabilityId}`;
  }

  /** GET /dental_clinics/:clinic_id/capabilities */
  getForClinic(clinicId: number): Observable<ClinicCapabilityLinkRow[]> {
    return this.http.get<ClinicCapabilityLinkRow[]>(this.clinicCapsUrl(clinicId), { headers: this.authHeaders() });
  }

  /** POST /dental_clinics/:clinic_id/capabilities  Body: { capability_id } */
  addToClinic(clinicId: number, capabilityId: number): Observable<ClinicCapabilityLinkRow> {
    const body: AddClinicCapabilityBody = { capability_id: capabilityId };
    return this.http.post<ClinicCapabilityLinkRow>(this.clinicCapsUrl(clinicId), body, { headers: this.authHeaders() });
  }

  /** DELETE /dental_clinics/:clinic_id/capabilities/:capability_id  (204 No Content) */
  removeFromClinic(clinicId: number, capabilityId: number): Observable<void> {
    return this.http.delete<void>(this.clinicCapUrl(clinicId, capabilityId), { headers: this.authHeaders() }  );
  }

  /** PUT /dental_clinics/:clinic_id/capabilities  Body: { capability_ids: [...] } */
  setForClinic(clinicId: number, capabilityIds: number[]): Observable<ClinicCapabilityLinkRow[]> {
    const body: SetClinicCapabilitiesBody = { capability_ids: capabilityIds };
    return this.http.put<ClinicCapabilityLinkRow[]>(this.clinicCapsUrl(clinicId), body);
  }
}
