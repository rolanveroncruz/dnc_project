import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {map, Observable} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';


export interface ClinicCapabilitiesPageInfo{
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
  items: ClinicCapability[];
}
export interface ClinicCapability{
  id: number;
  name: string;
  active: boolean;
  last_modified_by: string;
  last_modified_on: Date;
}
export type CreateClinicCapabilityRequest = {
  name: string;
  active?: boolean;
};

export type PatchClinicCapabilityRequest = {
  name?: string;
  active?: boolean;
};
/**
 * ✅ DTO types describe what actually comes "over the wire" from the backend.
 * JSON does NOT contain Date objects; it contains strings. So we map string -> Date.
 *
 * You usually DO NOT reference these outside the service.
 */
type ClinicCapabilityDto = Omit<ClinicCapability, 'last_modified_on'> & {
  last_modified_on: string; // <-- backend sends ISO string (ex: "2026-01-13T12:34:56Z")
};
@Injectable({
  providedIn: 'root',
})
export class ClinicCapabilitiesService {
  private apiUrl = environment.apiUrl;


  constructor(private http: HttpClient, private LoginService:LoginService) {}

  private authHeaders() {
    let token = this.LoginService.token();
    return  {'Authorization': `Bearer ${token}`};
  }
  /**
   * ✅ Convert backend DTO -> frontend model
   * This is where we turn `last_modified_on` from a string into a Date.
   *
   * CHANGE HERE if:
   * - backend key names differ
   * - you want to keep it as string instead of Date
   */
  private toModel(dto: ClinicCapabilityDto): ClinicCapability {
    return {
      ...dto,
      last_modified_on: new Date(dto.last_modified_on), // <-- CHANGE if you prefer string
    };
  }

  getClinicCapabilities():Observable<ClinicCapabilitiesPageInfo>{
    return this.http.get<ClinicCapabilitiesPageInfo>(
      `${this.apiUrl}/api/clinic_capabilities?`,
      {
        headers:
          this.authHeaders()
      });
  }
  postClinicCapability(payload:CreateClinicCapabilityRequest):Observable<ClinicCapability>{
    return this.http
      .post<ClinicCapabilityDto>(
      `${this.apiUrl}/api/clinic_capabilities/`,
        payload,
        { headers: this.authHeaders(),
        })
        .pipe(map((dto)=>this.toModel(dto)));
  }


  patchClinicCapability(clinicCapabilityId:number, payload:PatchClinicCapabilityRequest)
  :Observable<ClinicCapability>{
    return this.http.patch<ClinicCapabilityDto>(
      `${this.apiUrl}/api/clinic_capabilities/${clinicCapabilityId}`,
      payload,
      { headers: this.authHeaders(),
      }
      )
      .pipe(map((dto)=>this.toModel(dto)));
  }

}
